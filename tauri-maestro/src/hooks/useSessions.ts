import { useEffect } from 'react';
import { useSessionStore, selectSessions, selectSessionCount } from '../stores';
import type { TerminalMode } from '../types';

/**
 * Hook for working with all sessions
 */
export function useSessions() {
  const sessions = useSessionStore(selectSessions);
  const sessionCount = useSessionStore(selectSessionCount);
  const isLoading = useSessionStore((s) => s.isLoading);
  const error = useSessionStore((s) => s.error);
  const fetchSessions = useSessionStore((s) => s.fetchSessions);
  const createSession = useSessionStore((s) => s.createSession);

  // Fetch sessions on mount
  useEffect(() => {
    fetchSessions();
  }, [fetchSessions]);

  const create = async (mode?: TerminalMode, workingDirectory?: string) => {
    return createSession(mode, workingDirectory);
  };

  return {
    sessions,
    sessionCount,
    isLoading,
    error,
    create,
    refresh: fetchSessions,
  };
}
