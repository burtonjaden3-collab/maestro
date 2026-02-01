use tauri::State;

use crate::core::process_manager::ProcessManager;
use crate::core::session_manager::{AiMode, SessionConfig, SessionManager, SessionStatus};

/// Exposes `SessionManager::all_sessions` to the frontend.
/// Returns a snapshot of all active sessions in arbitrary order.
#[tauri::command]
pub async fn get_sessions(state: State<'_, SessionManager>) -> Result<Vec<SessionConfig>, String> {
    Ok(state.all_sessions())
}

/// Exposes `SessionManager::create_session` to the frontend.
/// Registers a new session with `Starting` status. Returns an error if the
/// session ID already exists.
#[tauri::command]
pub async fn create_session(
    state: State<'_, SessionManager>,
    id: u32,
    mode: AiMode,
    project_path: String,
) -> Result<SessionConfig, String> {
    // Canonicalize path for consistent storage
    let canonical = std::fs::canonicalize(&project_path)
        .map_err(|e| format!("Invalid project path '{}': {}", project_path, e))?
        .to_string_lossy()
        .into_owned();

    state.create_session(id, mode, canonical)
        .map_err(|existing| format!("Session {} already exists", existing.id))
}

/// Exposes `SessionManager::update_status` to the frontend.
/// Returns `false` if the session does not exist (no error raised).
#[tauri::command]
pub async fn update_session_status(
    state: State<'_, SessionManager>,
    session_id: u32,
    status: SessionStatus,
) -> Result<bool, String> {
    Ok(state.update_status(session_id, status))
}

/// Exposes `SessionManager::assign_branch` to the frontend.
/// Links a session to a branch and optional worktree path. Returns an error
/// string if the session does not exist.
#[tauri::command]
pub async fn assign_session_branch(
    state: State<'_, SessionManager>,
    session_id: u32,
    branch: String,
    worktree_path: Option<String>,
) -> Result<SessionConfig, String> {
    state
        .assign_branch(session_id, branch, worktree_path)
        .ok_or_else(|| format!("Session {} not found", session_id))
}

/// Exposes `SessionManager::remove_session` to the frontend.
/// Returns the removed session config, or `None` if it was not found.
#[tauri::command]
pub async fn remove_session(
    state: State<'_, SessionManager>,
    session_id: u32,
) -> Result<Option<SessionConfig>, String> {
    Ok(state.remove_session(session_id))
}

/// Gets all sessions for a specific project.
#[tauri::command]
pub async fn get_sessions_for_project(
    state: State<'_, SessionManager>,
    project_path: String,
) -> Result<Vec<SessionConfig>, String> {
    let canonical = std::fs::canonicalize(&project_path)
        .map_err(|e| format!("Invalid project path '{}': {}", project_path, e))?
        .to_string_lossy()
        .into_owned();

    Ok(state.get_sessions_for_project(&canonical))
}

/// Removes all sessions for a project (used when closing a project tab).
/// Also kills the associated PTY sessions.
#[tauri::command]
pub async fn remove_sessions_for_project(
    state: State<'_, SessionManager>,
    process_manager: State<'_, ProcessManager>,
    project_path: String,
) -> Result<Vec<SessionConfig>, String> {
    let canonical = std::fs::canonicalize(&project_path)
        .map_err(|e| format!("Invalid project path '{}': {}", project_path, e))?
        .to_string_lossy()
        .into_owned();

    let removed = state.remove_sessions_for_project(&canonical);

    // Also kill associated PTY sessions
    for session in &removed {
        // Fire-and-forget kill -- log errors but don't fail the removal
        if let Err(e) = process_manager.kill_session(session.id).await {
            log::warn!("Failed to kill PTY for session {}: {}", session.id, e);
        }
    }

    Ok(removed)
}
