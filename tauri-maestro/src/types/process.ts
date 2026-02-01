// Status of a managed process
export type ManagedProcessStatus =
  | 'starting'
  | 'running'
  | 'stopping'
  | 'stopped'
  | 'error';

// Source/type of process
export type ProcessSource = 'terminal' | 'devServer' | 'background' | 'system';

// A process tracked by the registry
export interface ManagedProcess {
  sessionId: string;
  pid: number;
  pgid: number;
  source: ProcessSource;
  command: string;
  status: ManagedProcessStatus;
  port: number | null;
  serverUrl: string | null;
}
