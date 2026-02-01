import { useEffect } from 'react';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { useSessionStore } from '../stores';
import type { Session } from '../types';

// Event payload types matching Rust definitions
interface SessionCreatedPayload {
  session: Session;
}

interface SessionUpdatedPayload {
  session: Session;
  changedFields: string[];
}

interface SessionStatusChangedPayload {
  sessionId: string;
  oldStatus: string;
  newStatus: string;
}

interface SessionStoppedPayload {
  sessionId: string;
  exitCode: number | null;
  reason: string;
}

interface SessionServerDetectedPayload {
  sessionId: string;
  url: string;
  port: number;
}

interface SessionDeletedPayload {
  sessionId: string;
}

/**
 * Hook that sets up Tauri event listeners for session updates
 */
export function useTauriEvents() {
  const handleSessionCreated = useSessionStore((s) => s.handleSessionCreated);
  const handleSessionUpdated = useSessionStore((s) => s.handleSessionUpdated);
  const handleSessionDeleted = useSessionStore((s) => s.handleSessionDeleted);

  useEffect(() => {
    const unlisteners: UnlistenFn[] = [];

    // Listen for session-created
    listen<SessionCreatedPayload>('session-created', (event) => {
      handleSessionCreated(event.payload.session);
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for session-updated
    listen<SessionUpdatedPayload>('session-updated', (event) => {
      handleSessionUpdated(event.payload.session);
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for session-status-changed
    listen<SessionStatusChangedPayload>('session-status-changed', (event) => {
      console.log(
        `Session ${event.payload.sessionId} status: ${event.payload.oldStatus} -> ${event.payload.newStatus}`
      );
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for session-stopped
    listen<SessionStoppedPayload>('session-stopped', (event) => {
      console.log(
        `Session ${event.payload.sessionId} stopped: ${event.payload.reason}`
      );
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for session-server-detected
    listen<SessionServerDetectedPayload>('session-server-detected', (event) => {
      console.log(
        `Session ${event.payload.sessionId} server detected: ${event.payload.url}`
      );
    }).then((unlisten) => unlisteners.push(unlisten));

    // Listen for session-deleted
    listen<SessionDeletedPayload>('session-deleted', (event) => {
      handleSessionDeleted(event.payload.sessionId);
    }).then((unlisten) => unlisteners.push(unlisten));

    // Cleanup
    return () => {
      unlisteners.forEach((unlisten) => unlisten());
    };
  }, [handleSessionCreated, handleSessionUpdated, handleSessionDeleted]);
}
