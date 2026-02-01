import { useCallback } from 'react';
import { useSessionStore, selectActiveSession } from '../stores';
import type { SessionUpdate } from '../types';

/**
 * Hook for working with the currently active session
 */
export function useActiveSession() {
  const activeSession = useSessionStore(selectActiveSession);
  const activeSessionId = useSessionStore((s) => s.activeSessionId);
  const setActiveSession = useSessionStore((s) => s.setActiveSession);
  const updateSession = useSessionStore((s) => s.updateSession);
  const deleteSession = useSessionStore((s) => s.deleteSession);
  const spawnSessionPty = useSessionStore((s) => s.spawnSessionPty);

  const update = useCallback(
    (updates: SessionUpdate) => {
      if (activeSessionId) {
        return updateSession(activeSessionId, updates);
      }
    },
    [activeSessionId, updateSession]
  );

  const remove = useCallback(() => {
    if (activeSessionId) {
      return deleteSession(activeSessionId);
    }
  }, [activeSessionId, deleteSession]);

  const spawnPty = useCallback(
    (workingDirectory?: string) => {
      if (activeSessionId) {
        return spawnSessionPty(activeSessionId, workingDirectory);
      }
    },
    [activeSessionId, spawnSessionPty]
  );

  return {
    session: activeSession,
    sessionId: activeSessionId,
    setActive: setActiveSession,
    update,
    remove,
    spawnPty,
    hasActive: !!activeSession,
  };
}
