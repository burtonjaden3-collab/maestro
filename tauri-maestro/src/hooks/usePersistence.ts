import { useEffect, useState } from 'react';
import { useSessionStore } from '../stores';

/**
 * Hook that handles hydrating the store from persistence on app startup
 */
export function usePersistence() {
  const [isHydrated, setIsHydrated] = useState(false);
  const fetchSessions = useSessionStore((s) => s.fetchSessions);

  useEffect(() => {
    // Fetch sessions from backend (which loads from persistence)
    fetchSessions().then(() => {
      setIsHydrated(true);
    });
  }, [fetchSessions]);

  return { isHydrated };
}
