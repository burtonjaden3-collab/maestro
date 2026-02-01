use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use super::types::{Session, TerminalMode};

const STORE_PATH: &str = "sessions.json";
const SESSIONS_KEY: &str = "sessions";

/// Minimal session data for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedSession {
    pub id: String,
    pub numeric_id: i32,
    pub mode: TerminalMode,
    pub working_directory: Option<String>,
    pub assigned_branch: Option<String>,
    pub custom_run_command: Option<String>,
}

impl From<&Session> for PersistedSession {
    fn from(session: &Session) -> Self {
        Self {
            id: session.id.clone(),
            numeric_id: session.numeric_id,
            mode: session.mode,
            working_directory: session.working_directory.clone(),
            assigned_branch: session.assigned_branch.clone(),
            custom_run_command: session.custom_run_command.clone(),
        }
    }
}

impl PersistedSession {
    pub fn into_session(self) -> Session {
        let mut session = Session::new(self.numeric_id);
        session.id = self.id;
        session.mode = self.mode;
        session.working_directory = self.working_directory;
        session.assigned_branch = self.assigned_branch;
        session.custom_run_command = self.custom_run_command;
        session
    }
}

/// Load sessions from the store
pub fn load_sessions(app: &AppHandle) -> Vec<Session> {
    let store = match app.store(STORE_PATH) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to open store: {}", e);
            return Vec::new();
        }
    };

    match store.get(SESSIONS_KEY) {
        Some(value) => {
            match serde_json::from_value::<Vec<PersistedSession>>(value.clone()) {
                Ok(persisted) => persisted.into_iter().map(|p| p.into_session()).collect(),
                Err(e) => {
                    eprintln!("Failed to deserialize sessions: {}", e);
                    Vec::new()
                }
            }
        }
        None => Vec::new(),
    }
}

/// Save sessions to the store
pub fn save_sessions(app: &AppHandle, sessions: &[Session]) -> Result<(), String> {
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;

    let persisted: Vec<PersistedSession> = sessions.iter().map(PersistedSession::from).collect();

    store.set(
        SESSIONS_KEY.to_string(),
        serde_json::to_value(&persisted).map_err(|e| e.to_string())?,
    );

    store.save().map_err(|e| e.to_string())
}

/// Clear all persisted sessions
pub fn clear_sessions(app: &AppHandle) -> Result<(), String> {
    let store = app.store(STORE_PATH).map_err(|e| e.to_string())?;
    store.delete(SESSIONS_KEY.to_string());
    store.save().map_err(|e| e.to_string())
}
