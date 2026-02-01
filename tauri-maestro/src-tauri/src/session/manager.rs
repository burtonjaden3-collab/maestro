use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use tauri::AppHandle;

use super::events::SessionEventEmitter;
use super::types::{Session, SessionStatus, SessionUpdate, TerminalMode};
use crate::process::ProcessRegistry;

/// Manages all sessions in the application
pub struct SessionManager {
    sessions: RwLock<HashMap<String, Session>>,
    next_numeric_id: AtomicI32,
    process_registry: ProcessRegistry,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            next_numeric_id: AtomicI32::new(1),
            process_registry: ProcessRegistry::new(),
        }
    }

    /// Create a new session
    pub fn create_session(&self, mode: Option<TerminalMode>, app: &AppHandle) -> Session {
        let numeric_id = self.next_numeric_id.fetch_add(1, Ordering::SeqCst);
        let mut session = Session::new(numeric_id);

        if let Some(m) = mode {
            session.mode = m;
        }

        {
            let mut sessions = self.sessions.write();
            sessions.insert(session.id.clone(), session.clone());
        }

        app.emit_session_created(&session);
        session
    }

    /// Create a session with a specific working directory
    pub fn create_session_with_directory(
        &self,
        mode: Option<TerminalMode>,
        working_dir: String,
        app: &AppHandle,
    ) -> Session {
        let numeric_id = self.next_numeric_id.fetch_add(1, Ordering::SeqCst);
        let mut session = Session::new(numeric_id).with_working_directory(working_dir);

        if let Some(m) = mode {
            session.mode = m;
        }

        {
            let mut sessions = self.sessions.write();
            sessions.insert(session.id.clone(), session.clone());
        }

        app.emit_session_created(&session);
        session
    }

    /// Get a session by ID
    pub fn get_session(&self, session_id: &str) -> Option<Session> {
        self.sessions.read().get(session_id).cloned()
    }

    /// Get all sessions
    pub fn list_sessions(&self) -> Vec<Session> {
        let sessions = self.sessions.read();
        let mut list: Vec<_> = sessions.values().cloned().collect();
        list.sort_by_key(|s| s.numeric_id);
        list
    }

    /// Update a session
    pub fn update_session(
        &self,
        session_id: &str,
        update: SessionUpdate,
        app: &AppHandle,
    ) -> Option<Session> {
        let (session, changed_fields, status_change) = {
            let mut sessions = self.sessions.write();
            if let Some(session) = sessions.get_mut(session_id) {
                let old_status = session.status;
                let changed = update.apply_to(session);

                let status_change = if changed.contains(&"status".to_string()) {
                    Some((old_status, session.status))
                } else {
                    None
                };

                (Some(session.clone()), changed, status_change)
            } else {
                (None, Vec::new(), None)
            }
        };

        if let Some(ref s) = session {
            if !changed_fields.is_empty() {
                app.emit_session_updated(s, changed_fields);
            }

            if let Some((old, new)) = status_change {
                app.emit_session_status_changed(&s.id, old, new);
            }
        }

        session
    }

    /// Update session status specifically
    pub fn update_status(
        &self,
        session_id: &str,
        status: SessionStatus,
        app: &AppHandle,
    ) -> Option<Session> {
        self.update_session(
            session_id,
            SessionUpdate {
                status: Some(status),
                ..Default::default()
            },
            app,
        )
    }

    /// Set terminal PID for a session
    pub fn set_terminal_pid(&self, session_id: &str, pid: u32, app: &AppHandle) -> Option<Session> {
        self.update_session(
            session_id,
            SessionUpdate {
                terminal_pid: Some(pid),
                is_terminal_launched: Some(true),
                ..Default::default()
            },
            app,
        )
    }

    /// Set server URL for a session
    pub fn set_server_url(
        &self,
        session_id: &str,
        url: String,
        port: u16,
        app: &AppHandle,
    ) -> Option<Session> {
        app.emit_session_server_detected(session_id, &url, port);

        self.update_session(
            session_id,
            SessionUpdate {
                server_url: Some(url),
                assigned_port: Some(port),
                ..Default::default()
            },
            app,
        )
    }

    /// Delete a session and cleanup resources
    pub fn delete_session(&self, session_id: &str, app: &AppHandle) -> Option<Session> {
        // Remove from registry
        let session = {
            let mut sessions = self.sessions.write();
            sessions.remove(session_id)
        };

        if session.is_some() {
            // Cleanup processes
            self.process_registry.remove_session(session_id);
            app.emit_session_deleted(session_id);
        }

        session
    }

    /// Mark a session as stopped
    pub fn session_stopped(
        &self,
        session_id: &str,
        exit_code: Option<i32>,
        reason: &str,
        app: &AppHandle,
    ) {
        app.emit_session_stopped(session_id, exit_code, reason);

        let _ = self.update_session(
            session_id,
            SessionUpdate {
                status: Some(SessionStatus::Done),
                is_terminal_launched: Some(false),
                is_cli_running: Some(false),
                terminal_pid: None,
                ..Default::default()
            },
            app,
        );
    }

    /// Get the process registry
    pub fn process_registry(&self) -> &ProcessRegistry {
        &self.process_registry
    }

    /// Restore sessions from persistence (called at startup)
    pub fn restore_sessions(&self, sessions: Vec<Session>, app: &AppHandle) {
        let mut max_numeric_id = 0;

        {
            let mut stored = self.sessions.write();
            for session in sessions {
                if session.numeric_id > max_numeric_id {
                    max_numeric_id = session.numeric_id;
                }
                stored.insert(session.id.clone(), session.clone());
                app.emit_session_created(&session);
            }
        }

        // Update next ID counter
        self.next_numeric_id
            .store(max_numeric_id + 1, Ordering::SeqCst);
    }

    /// Get sessions for persistence (excludes runtime state)
    pub fn get_persistable_sessions(&self) -> Vec<Session> {
        self.list_sessions()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
