use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Status of a session in the lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SessionStatus {
    #[default]
    Initializing,
    Idle,
    Working,
    Waiting,
    Done,
    Error,
}

impl SessionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SessionStatus::Initializing => "initializing",
            SessionStatus::Idle => "idle",
            SessionStatus::Working => "working",
            SessionStatus::Waiting => "waiting",
            SessionStatus::Done => "done",
            SessionStatus::Error => "error",
        }
    }
}

/// Terminal mode determines what CLI tool runs in the session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum TerminalMode {
    #[default]
    ClaudeCode,
    GeminiCli,
    OpenAiCodex,
    PlainTerminal,
}

impl TerminalMode {
    pub fn command(&self) -> &'static str {
        match self {
            TerminalMode::ClaudeCode => "claude",
            TerminalMode::GeminiCli => "gemini",
            TerminalMode::OpenAiCodex => "codex",
            TerminalMode::PlainTerminal => "",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TerminalMode::ClaudeCode => "Claude Code",
            TerminalMode::GeminiCli => "Gemini CLI",
            TerminalMode::OpenAiCodex => "OpenAI Codex",
            TerminalMode::PlainTerminal => "Terminal",
        }
    }
}

/// A session represents a single terminal/agent workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// Unique identifier (UUID)
    pub id: String,
    /// Display ID for UI (1, 2, 3...)
    pub numeric_id: i32,
    /// Current status
    pub status: SessionStatus,
    /// Terminal mode (Claude, Gemini, etc.)
    pub mode: TerminalMode,
    /// Working directory path
    pub working_directory: Option<String>,
    /// Assigned git branch
    pub assigned_branch: Option<String>,
    /// PID of the terminal process
    pub terminal_pid: Option<u32>,
    /// Whether terminal has been spawned
    pub is_terminal_launched: bool,
    /// Whether the CLI tool is running
    pub is_cli_running: bool,
    /// Assigned port for dev server
    pub assigned_port: Option<u16>,
    /// Detected server URL
    pub server_url: Option<String>,
    /// Custom run command override
    pub custom_run_command: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
}

impl Session {
    pub fn new(numeric_id: i32) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            numeric_id,
            status: SessionStatus::Initializing,
            mode: TerminalMode::ClaudeCode,
            working_directory: None,
            assigned_branch: None,
            terminal_pid: None,
            is_terminal_launched: false,
            is_cli_running: false,
            assigned_port: None,
            server_url: None,
            custom_run_command: None,
            created_at: now,
            last_activity: now,
        }
    }

    pub fn with_mode(mut self, mode: TerminalMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_working_directory(mut self, dir: String) -> Self {
        self.working_directory = Some(dir);
        self
    }

    pub fn touch(&mut self) {
        self.last_activity = Utc::now();
    }
}

/// Partial update for session fields
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUpdate {
    pub status: Option<SessionStatus>,
    pub mode: Option<TerminalMode>,
    pub working_directory: Option<String>,
    pub assigned_branch: Option<String>,
    pub terminal_pid: Option<u32>,
    pub is_terminal_launched: Option<bool>,
    pub is_cli_running: Option<bool>,
    pub assigned_port: Option<u16>,
    pub server_url: Option<String>,
    pub custom_run_command: Option<String>,
}

impl SessionUpdate {
    pub fn apply_to(&self, session: &mut Session) -> Vec<String> {
        let mut changed = Vec::new();

        if let Some(status) = self.status {
            if session.status != status {
                session.status = status;
                changed.push("status".to_string());
            }
        }
        if let Some(mode) = self.mode {
            if session.mode != mode {
                session.mode = mode;
                changed.push("mode".to_string());
            }
        }
        if let Some(ref dir) = self.working_directory {
            session.working_directory = Some(dir.clone());
            changed.push("workingDirectory".to_string());
        }
        if let Some(ref branch) = self.assigned_branch {
            session.assigned_branch = Some(branch.clone());
            changed.push("assignedBranch".to_string());
        }
        if let Some(pid) = self.terminal_pid {
            session.terminal_pid = Some(pid);
            changed.push("terminalPid".to_string());
        }
        if let Some(launched) = self.is_terminal_launched {
            if session.is_terminal_launched != launched {
                session.is_terminal_launched = launched;
                changed.push("isTerminalLaunched".to_string());
            }
        }
        if let Some(running) = self.is_cli_running {
            if session.is_cli_running != running {
                session.is_cli_running = running;
                changed.push("isCliRunning".to_string());
            }
        }
        if let Some(port) = self.assigned_port {
            session.assigned_port = Some(port);
            changed.push("assignedPort".to_string());
        }
        if let Some(ref url) = self.server_url {
            session.server_url = Some(url.clone());
            changed.push("serverUrl".to_string());
        }
        if let Some(ref cmd) = self.custom_run_command {
            session.custom_run_command = Some(cmd.clone());
            changed.push("customRunCommand".to_string());
        }

        if !changed.is_empty() {
            session.touch();
        }

        changed
    }
}
