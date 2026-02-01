import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

/**
 * PTY service for interacting with terminal processes
 */
export const ptyService = {
  /**
   * Spawn a new PTY for a session
   */
  spawn: async (sessionId: string): Promise<void> => {
    await invoke('spawn_pty', { sessionId });
  },

  /**
   * Spawn a PTY with session integration (returns PID)
   */
  spawnSession: async (
    sessionId: string,
    workingDirectory?: string
  ): Promise<number> => {
    return invoke<number>('spawn_session_pty', {
      sessionId,
      workingDirectory,
    });
  },

  /**
   * Write data to a PTY
   */
  write: async (sessionId: string, data: string): Promise<void> => {
    await invoke('write_pty', { sessionId, data });
  },

  /**
   * Resize a PTY
   */
  resize: async (
    sessionId: string,
    cols: number,
    rows: number
  ): Promise<void> => {
    await invoke('resize_pty', { sessionId, cols, rows });
  },

  /**
   * Kill a PTY
   */
  kill: async (sessionId: string): Promise<void> => {
    await invoke('kill_pty', { sessionId });
  },

  /**
   * Listen for PTY output
   */
  onOutput: (
    sessionId: string,
    callback: (data: string) => void
  ): Promise<UnlistenFn> => {
    return listen<string>(`pty-output-${sessionId}`, (event) => {
      callback(event.payload);
    });
  },
};
