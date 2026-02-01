use std::sync::Arc;
use tauri::State;

use crate::process::ManagedProcess;
use crate::pty::PtyManager;
use crate::session::{
    save_sessions, Session, SessionManager, SessionStatus, SessionUpdate, TerminalMode,
};

/// List all sessions
#[tauri::command]
pub async fn list_sessions(state: State<'_, Arc<SessionManager>>) -> Result<Vec<Session>, String> {
    Ok(state.list_sessions())
}

/// Get a single session by ID
#[tauri::command]
pub async fn get_session(
    session_id: String,
    state: State<'_, Arc<SessionManager>>,
) -> Result<Option<Session>, String> {
    Ok(state.get_session(&session_id))
}

/// Create a new session
#[tauri::command]
pub async fn create_session(
    mode: Option<TerminalMode>,
    working_directory: Option<String>,
    state: State<'_, Arc<SessionManager>>,
    app: tauri::AppHandle,
) -> Result<Session, String> {
    let session = if let Some(dir) = working_directory {
        state.create_session_with_directory(mode, dir, &app)
    } else {
        state.create_session(mode, &app)
    };

    // Persist sessions
    let sessions = state.get_persistable_sessions();
    let _ = save_sessions(&app, &sessions);

    Ok(session)
}

/// Update a session
#[tauri::command]
pub async fn update_session(
    session_id: String,
    status: Option<SessionStatus>,
    mode: Option<TerminalMode>,
    working_directory: Option<String>,
    assigned_branch: Option<String>,
    assigned_port: Option<u16>,
    server_url: Option<String>,
    custom_run_command: Option<String>,
    is_cli_running: Option<bool>,
    state: State<'_, Arc<SessionManager>>,
    app: tauri::AppHandle,
) -> Result<Option<Session>, String> {
    let update = SessionUpdate {
        status,
        mode,
        working_directory,
        assigned_branch,
        terminal_pid: None, // Don't allow direct PID updates via command
        is_terminal_launched: None,
        is_cli_running,
        assigned_port,
        server_url,
        custom_run_command,
    };

    let session = state.update_session(&session_id, update, &app);

    // Persist sessions
    let sessions = state.get_persistable_sessions();
    let _ = save_sessions(&app, &sessions);

    Ok(session)
}

/// Delete a session
#[tauri::command]
pub async fn delete_session(
    session_id: String,
    state: State<'_, Arc<SessionManager>>,
    pty_state: State<'_, Arc<PtyManager>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Kill PTY first
    let _ = pty_state.kill(&session_id).await;

    // Delete session
    state.delete_session(&session_id, &app);

    // Persist sessions
    let sessions = state.get_persistable_sessions();
    let _ = save_sessions(&app, &sessions);

    Ok(())
}

/// Spawn a PTY for a session
#[tauri::command]
pub async fn spawn_session_pty(
    session_id: String,
    working_directory: Option<String>,
    session_state: State<'_, Arc<SessionManager>>,
    pty_state: State<'_, Arc<PtyManager>>,
    app: tauri::AppHandle,
) -> Result<u32, String> {
    // Determine working directory
    let work_dir = working_directory.or_else(|| {
        session_state
            .get_session(&session_id)
            .and_then(|s| s.working_directory)
    });

    // Spawn the PTY
    let pid = pty_state
        .spawn_with_callback(&session_id, work_dir, app.clone(), {
            let session_manager = Arc::clone(&session_state.inner());
            let app_handle = app.clone();
            let sid = session_id.clone();
            move |output| {
                // Check for URL detection
                if let Some(server) = crate::url_detection::detect_server_url(output) {
                    session_manager.set_server_url(&sid, server.url, server.port, &app_handle);
                }
            }
        })
        .await
        .map_err(|e| e.to_string())?;

    // Update session with PID
    session_state.set_terminal_pid(&session_id, pid, &app);

    // Persist sessions
    let sessions = session_state.get_persistable_sessions();
    let _ = save_sessions(&app, &sessions);

    Ok(pid)
}

/// Get all processes for a session
#[tauri::command]
pub async fn get_session_processes(
    session_id: String,
    state: State<'_, Arc<SessionManager>>,
) -> Result<Vec<ManagedProcess>, String> {
    Ok(state.process_registry().get_session_processes(&session_id))
}

// Legacy PTY commands for backward compatibility

#[tauri::command]
pub async fn spawn_pty(
    session_id: String,
    state: tauri::State<'_, Arc<PtyManager>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    state
        .spawn(&session_id, app)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn write_pty(
    session_id: String,
    data: String,
    state: tauri::State<'_, Arc<PtyManager>>,
) -> Result<(), String> {
    state
        .write(&session_id, data.as_bytes())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn resize_pty(
    session_id: String,
    cols: u16,
    rows: u16,
    state: tauri::State<'_, Arc<PtyManager>>,
) -> Result<(), String> {
    state
        .resize(&session_id, cols, rows)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn kill_pty(
    session_id: String,
    state: tauri::State<'_, Arc<PtyManager>>,
) -> Result<(), String> {
    state.kill(&session_id).await.map_err(|e| e.to_string())
}
