// Agent state reported via MCP
export type AgentState =
  | 'idle'
  | 'working'
  | 'needs_input'
  | 'finished'
  | 'error';

// Agent status update from MCP
export interface AgentStatus {
  sessionId: string;
  state: AgentState;
  message: string;
  needsInputPrompt?: string;
  timestamp: number;
}

// Mapping agent states to session statuses
export const AGENT_STATE_TO_SESSION_STATUS: Record<
  AgentState,
  import('./session').SessionStatus
> = {
  idle: 'idle',
  working: 'working',
  needs_input: 'waiting',
  finished: 'done',
  error: 'error',
};

// Agent state configuration
export const AGENT_STATE_CONFIG: Record<
  AgentState,
  { label: string; color: string; showPrompt: boolean }
> = {
  idle: {
    label: 'Idle',
    color: '#6b7280',
    showPrompt: false,
  },
  working: {
    label: 'Working',
    color: '#f97316',
    showPrompt: false,
  },
  needs_input: {
    label: 'Needs Input',
    color: '#eab308',
    showPrompt: true,
  },
  finished: {
    label: 'Finished',
    color: '#22c55e',
    showPrompt: false,
  },
  error: {
    label: 'Error',
    color: '#ef4444',
    showPrompt: false,
  },
};
