use serde::{Deserialize, Serialize};

/// Status of a managed process
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ManagedProcessStatus {
    #[default]
    Starting,
    Running,
    Stopping,
    Stopped,
    Error,
}

/// Source/type of process
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProcessSource {
    Terminal,
    DevServer,
    Background,
    System,
}

/// A process tracked by the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedProcess {
    /// Session this process belongs to
    pub session_id: String,
    /// Process ID
    pub pid: u32,
    /// Process group ID (Unix only)
    pub pgid: u32,
    /// Source of the process
    pub source: ProcessSource,
    /// Command that started this process
    pub command: String,
    /// Current status
    pub status: ManagedProcessStatus,
    /// Port if this is a server process
    pub port: Option<u16>,
    /// Server URL if detected
    pub server_url: Option<String>,
}

impl ManagedProcess {
    pub fn new(session_id: String, pid: u32, source: ProcessSource, command: String) -> Self {
        Self {
            session_id,
            pid,
            pgid: pid, // Default to same as PID, updated on Unix
            source,
            command,
            status: ManagedProcessStatus::Starting,
            port: None,
            server_url: None,
        }
    }

    pub fn with_pgid(mut self, pgid: u32) -> Self {
        self.pgid = pgid;
        self
    }

    pub fn set_running(&mut self) {
        self.status = ManagedProcessStatus::Running;
    }

    pub fn set_stopping(&mut self) {
        self.status = ManagedProcessStatus::Stopping;
    }

    pub fn set_stopped(&mut self) {
        self.status = ManagedProcessStatus::Stopped;
    }

    pub fn set_error(&mut self) {
        self.status = ManagedProcessStatus::Error;
    }

    pub fn set_server(&mut self, port: u16, url: String) {
        self.port = Some(port);
        self.server_url = Some(url);
    }
}
