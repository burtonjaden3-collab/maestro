import { useCallback } from 'react';
import { useSessionStore, selectSession } from '../stores';
import type { SessionUpdate } from '../types';

/**
 * Hook for working with a single session by ID
 */
export function useSession(sessionId: string) {
  const session = useSessionStore(selectSession(sessionId));
  const updateSession = useSessionStore((s) => s.updateSession);
  const deleteSession = useSessionStore((s) => s.deleteSession);
  const spawnSessionPty = useSessionStore((s) => s.spawnSessionPty);

  const update = useCallback(
    (updates: SessionUpdate) => updateSession(sessionId, updates),
    [sessionId, updateSession]
  );

  const remove = useCallback(
    () => deleteSession(sessionId),
    [sessionId, deleteSession]
  );

  const spawnPty = useCallback(
    (workingDirectory?: string) => spawnSessionPty(sessionId, workingDirectory),
    [sessionId, spawnSessionPty]
  );

  return {
    session,
    update,
    remove,
    spawnPty,
    exists: !!session,
  };
}
