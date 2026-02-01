// Session status in lifecycle
export type SessionStatus =
  | 'initializing'
  | 'idle'
  | 'working'
  | 'waiting'
  | 'done'
  | 'error';

// Terminal mode determines what CLI tool runs
export type TerminalMode =
  | 'claudeCode'
  | 'geminiCli'
  | 'openAiCodex'
  | 'plainTerminal';

// A session represents a single terminal/agent workspace
export interface Session {
  id: string;
  numericId: number;
  status: SessionStatus;
  mode: TerminalMode;
  workingDirectory: string | null;
  assignedBranch: string | null;
  terminalPid: number | null;
  isTerminalLaunched: boolean;
  isCliRunning: boolean;
  assignedPort: number | null;
  serverUrl: string | null;
  customRunCommand: string | null;
  createdAt: number; // Unix timestamp
  lastActivity: number; // Unix timestamp
}

// Partial update for session fields
export interface SessionUpdate {
  status?: SessionStatus;
  mode?: TerminalMode;
  workingDirectory?: string;
  assignedBranch?: string;
  assignedPort?: number;
  serverUrl?: string;
  customRunCommand?: string;
  isCliRunning?: boolean;
}

// Terminal mode configuration
export const TERMINAL_MODES: Record<
  TerminalMode,
  { command: string; displayName: string; color: string }
> = {
  claudeCode: {
    command: 'claude',
    displayName: 'Claude Code',
    color: '#f97316', // Orange
  },
  geminiCli: {
    command: 'gemini',
    displayName: 'Gemini CLI',
    color: '#3b82f6', // Blue
  },
  openAiCodex: {
    command: 'codex',
    displayName: 'OpenAI Codex',
    color: '#10b981', // Green
  },
  plainTerminal: {
    command: '',
    displayName: 'Terminal',
    color: '#6b7280', // Gray
  },
};

// Session status configuration
export const SESSION_STATUS_CONFIG: Record<
  SessionStatus,
  { label: string; color: string; icon: string }
> = {
  initializing: {
    label: 'Initializing',
    color: '#9ca3af',
    icon: '⏳',
  },
  idle: {
    label: 'Idle',
    color: '#6b7280',
    icon: '💤',
  },
  working: {
    label: 'Working',
    color: '#f97316',
    icon: '🔧',
  },
  waiting: {
    label: 'Waiting',
    color: '#eab308',
    icon: '⏸️',
  },
  done: {
    label: 'Done',
    color: '#22c55e',
    icon: '✅',
  },
  error: {
    label: 'Error',
    color: '#ef4444',
    icon: '❌',
  },
};
