use serde::Serialize;
use tauri::{AppHandle, Emitter};

use super::types::{Session, SessionStatus};

/// Event payload for session creation
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionCreatedPayload {
    pub session: Session,
}

/// Event payload for session updates
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUpdatedPayload {
    pub session: Session,
    pub changed_fields: Vec<String>,
}

/// Event payload for status changes
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionStatusChangedPayload {
    pub session_id: String,
    pub old_status: SessionStatus,
    pub new_status: SessionStatus,
}

/// Event payload for session termination
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionStoppedPayload {
    pub session_id: String,
    pub exit_code: Option<i32>,
    pub reason: String,
}

/// Event payload for server detection
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionServerDetectedPayload {
    pub session_id: String,
    pub url: String,
    pub port: u16,
}

/// Event payload for session deletion
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDeletedPayload {
    pub session_id: String,
}

/// Helper trait for emitting session events
pub trait SessionEventEmitter {
    fn emit_session_created(&self, session: &Session);
    fn emit_session_updated(&self, session: &Session, changed_fields: Vec<String>);
    fn emit_session_status_changed(
        &self,
        session_id: &str,
        old_status: SessionStatus,
        new_status: SessionStatus,
    );
    fn emit_session_stopped(&self, session_id: &str, exit_code: Option<i32>, reason: &str);
    fn emit_session_server_detected(&self, session_id: &str, url: &str, port: u16);
    fn emit_session_deleted(&self, session_id: &str);
}

impl SessionEventEmitter for AppHandle {
    fn emit_session_created(&self, session: &Session) {
        let _ = self.emit(
            "session-created",
            SessionCreatedPayload {
                session: session.clone(),
            },
        );
    }

    fn emit_session_updated(&self, session: &Session, changed_fields: Vec<String>) {
        let _ = self.emit(
            "session-updated",
            SessionUpdatedPayload {
                session: session.clone(),
                changed_fields,
            },
        );
    }

    fn emit_session_status_changed(
        &self,
        session_id: &str,
        old_status: SessionStatus,
        new_status: SessionStatus,
    ) {
        let _ = self.emit(
            "session-status-changed",
            SessionStatusChangedPayload {
                session_id: session_id.to_string(),
                old_status,
                new_status,
            },
        );
    }

    fn emit_session_stopped(&self, session_id: &str, exit_code: Option<i32>, reason: &str) {
        let _ = self.emit(
            "session-stopped",
            SessionStoppedPayload {
                session_id: session_id.to_string(),
                exit_code,
                reason: reason.to_string(),
            },
        );
    }

    fn emit_session_server_detected(&self, session_id: &str, url: &str, port: u16) {
        let _ = self.emit(
            "session-server-detected",
            SessionServerDetectedPayload {
                session_id: session_id.to_string(),
                url: url.to_string(),
                port,
            },
        );
    }

    fn emit_session_deleted(&self, session_id: &str) {
        let _ = self.emit(
            "session-deleted",
            SessionDeletedPayload {
                session_id: session_id.to_string(),
            },
        );
    }
}
