import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { Session, SessionUpdate, TerminalMode } from '../types';

interface SessionState {
  sessions: Map<string, Session>;
  activeSessionId: string | null;
  isLoading: boolean;
  error: string | null;

  // Actions
  setActiveSession: (sessionId: string | null) => void;
  fetchSessions: () => Promise<void>;
  createSession: (mode?: TerminalMode, workingDirectory?: string) => Promise<Session>;
  updateSession: (sessionId: string, update: SessionUpdate) => Promise<void>;
  deleteSession: (sessionId: string) => Promise<void>;
  spawnSessionPty: (sessionId: string, workingDirectory?: string) => Promise<number>;

  // Internal actions for event handling
  handleSessionCreated: (session: Session) => void;
  handleSessionUpdated: (session: Session) => void;
  handleSessionDeleted: (sessionId: string) => void;
}

export const useSessionStore = create<SessionState>((set, get) => ({
  sessions: new Map(),
  activeSessionId: null,
  isLoading: false,
  error: null,

  setActiveSession: (sessionId) => {
    set({ activeSessionId: sessionId });
  },

  fetchSessions: async () => {
    set({ isLoading: true, error: null });
    try {
      const sessions = await invoke<Session[]>('list_sessions');
      const sessionMap = new Map<string, Session>();
      sessions.forEach((s) => sessionMap.set(s.id, s));
      set({ sessions: sessionMap, isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  createSession: async (mode, workingDirectory) => {
    try {
      const session = await invoke<Session>('create_session', {
        mode,
        workingDirectory,
      });
      const sessions = new Map(get().sessions);
      sessions.set(session.id, session);
      set({ sessions });
      return session;
    } catch (error) {
      set({ error: String(error) });
      throw error;
    }
  },

  updateSession: async (sessionId, update) => {
    try {
      const session = await invoke<Session | null>('update_session', {
        sessionId,
        ...update,
      });
      if (session) {
        const sessions = new Map(get().sessions);
        sessions.set(session.id, session);
        set({ sessions });
      }
    } catch (error) {
      set({ error: String(error) });
      throw error;
    }
  },

  deleteSession: async (sessionId) => {
    try {
      await invoke('delete_session', { sessionId });
      const sessions = new Map(get().sessions);
      sessions.delete(sessionId);

      // Clear active session if deleted
      const activeSessionId =
        get().activeSessionId === sessionId ? null : get().activeSessionId;

      set({ sessions, activeSessionId });
    } catch (error) {
      set({ error: String(error) });
      throw error;
    }
  },

  spawnSessionPty: async (sessionId, workingDirectory) => {
    try {
      const pid = await invoke<number>('spawn_session_pty', {
        sessionId,
        workingDirectory,
      });
      return pid;
    } catch (error) {
      set({ error: String(error) });
      throw error;
    }
  },

  // Event handlers
  handleSessionCreated: (session) => {
    const sessions = new Map(get().sessions);
    sessions.set(session.id, session);
    set({ sessions });
  },

  handleSessionUpdated: (session) => {
    const sessions = new Map(get().sessions);
    sessions.set(session.id, session);
    set({ sessions });
  },

  handleSessionDeleted: (sessionId) => {
    const sessions = new Map(get().sessions);
    sessions.delete(sessionId);

    const activeSessionId =
      get().activeSessionId === sessionId ? null : get().activeSessionId;

    set({ sessions, activeSessionId });
  },
}));

// Selectors
export const selectSessions = (state: SessionState) =>
  Array.from(state.sessions.values()).sort((a, b) => a.numericId - b.numericId);

export const selectSession = (sessionId: string) => (state: SessionState) =>
  state.sessions.get(sessionId);

export const selectActiveSession = (state: SessionState) =>
  state.activeSessionId ? state.sessions.get(state.activeSessionId) : null;

export const selectSessionsByStatus =
  (status: Session['status']) => (state: SessionState) =>
    Array.from(state.sessions.values()).filter((s) => s.status === status);

export const selectSessionCount = (state: SessionState) => state.sessions.size;
