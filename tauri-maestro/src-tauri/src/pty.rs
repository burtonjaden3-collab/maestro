use parking_lot::Mutex;
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};

struct PtySession {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    child: Box<dyn Child + Send + Sync>,
}

pub struct PtyManager {
    sessions: Mutex<HashMap<String, PtySession>>,
}

impl PtyManager {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    /// Spawn a PTY with an output callback for URL detection
    pub async fn spawn_with_callback<F>(
        &self,
        session_id: &str,
        working_directory: Option<String>,
        app: AppHandle,
        on_output: F,
    ) -> anyhow::Result<u32>
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        let pty_system = native_pty_system();

        let pair = pty_system.openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        // Get default shell
        let shell = Self::get_shell();
        let mut cmd = CommandBuilder::new(&shell);
        cmd.env("TERM", "xterm-256color");

        // Set working directory
        if let Some(dir) = working_directory {
            cmd.cwd(dir);
        } else if let Some(home) = dirs::home_dir() {
            cmd.cwd(home);
        }

        let child = pair.slave.spawn_command(cmd)?;

        // Get child PID
        #[cfg(unix)]
        let pid = child.process_id().unwrap_or(0);
        #[cfg(windows)]
        let pid = child.process_id().unwrap_or(0);

        let mut reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        // Store session
        {
            let mut sessions = self.sessions.lock();
            sessions.insert(
                session_id.to_string(),
                PtySession {
                    master: pair.master,
                    writer,
                    child,
                },
            );
        }

        // Spawn reader task with callback
        let event_name = format!("pty-output-{}", session_id);
        let session_id_owned = session_id.to_string();
        let on_output = Arc::new(on_output);

        std::thread::spawn(move || {
            let mut buf = [0u8; 8192];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let data = String::from_utf8_lossy(&buf[..n]).to_string();

                        // Call the output callback for URL detection
                        on_output(&data);

                        // Emit to frontend
                        let _ = app.emit(&event_name, &data);
                    }
                    Err(_) => break,
                }
            }
            println!("PTY reader exited for session {}", session_id_owned);
        });

        Ok(pid)
    }

    /// Legacy spawn without callback
    pub async fn spawn(&self, session_id: &str, app: AppHandle) -> anyhow::Result<()> {
        self.spawn_with_callback(session_id, None, app, |_| {})
            .await?;
        Ok(())
    }

    pub async fn write(&self, session_id: &str, data: &[u8]) -> anyhow::Result<()> {
        let mut sessions = self.sessions.lock();
        if let Some(session) = sessions.get_mut(session_id) {
            session.writer.write_all(data)?;
            session.writer.flush()?;
        }
        Ok(())
    }

    pub async fn resize(&self, session_id: &str, cols: u16, rows: u16) -> anyhow::Result<()> {
        let sessions = self.sessions.lock();
        if let Some(session) = sessions.get(session_id) {
            session.master.resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })?;
        }
        Ok(())
    }

    pub async fn kill(&self, session_id: &str) -> anyhow::Result<()> {
        let mut sessions = self.sessions.lock();
        if let Some(mut session) = sessions.remove(session_id) {
            // Try to kill the child process
            let _ = session.child.kill();
        }
        Ok(())
    }

    /// Get the PID of a session's child process
    pub fn get_pid(&self, session_id: &str) -> Option<u32> {
        let sessions = self.sessions.lock();
        sessions
            .get(session_id)
            .and_then(|s| s.child.process_id())
    }

    /// Check if a session's child is still running
    pub fn is_running(&self, session_id: &str) -> bool {
        let mut sessions = self.sessions.lock();
        if let Some(session) = sessions.get_mut(session_id) {
            // try_wait returns Ok(Some(status)) if exited, Ok(None) if still running
            matches!(session.child.try_wait(), Ok(None))
        } else {
            false
        }
    }

    fn get_shell() -> String {
        #[cfg(windows)]
        {
            std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
        }
        #[cfg(not(windows))]
        {
            std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string())
        }
    }
}

impl Default for PtyManager {
    fn default() -> Self {
        Self::new()
    }
}
