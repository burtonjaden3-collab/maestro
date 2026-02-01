mod commands;
mod process;
mod pty;
mod session;
mod url_detection;

use pty::PtyManager;
use session::{load_sessions, SessionManager};
use std::sync::Arc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let pty_manager = Arc::new(PtyManager::new());
    let session_manager = Arc::new(SessionManager::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(pty_manager)
        .manage(session_manager.clone())
        .setup(move |app| {
            // Restore sessions from persistence
            let sessions = load_sessions(&app.handle());
            if !sessions.is_empty() {
                session_manager.restore_sessions(sessions, &app.handle());
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Session management commands
            commands::list_sessions,
            commands::get_session,
            commands::create_session,
            commands::update_session,
            commands::delete_session,
            commands::spawn_session_pty,
            commands::get_session_processes,
            // Legacy PTY commands (backward compatibility)
            commands::spawn_pty,
            commands::write_pty,
            commands::resize_pty,
            commands::kill_pty,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
