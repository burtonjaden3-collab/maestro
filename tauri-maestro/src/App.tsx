import { useEffect, useState } from 'react';
import { Terminal } from './components/Terminal';
import { useSessions, useTauriEvents, usePersistence } from './hooks';
import { SESSION_STATUS_CONFIG, TERMINAL_MODES } from './types';
import type { Session } from './types';

const DEFAULT_SESSION_COUNT = 6;

function SessionCard({ session }: { session: Session }) {
  const statusConfig = SESSION_STATUS_CONFIG[session.status];
  const modeConfig = TERMINAL_MODES[session.mode];

  return (
    <div className="h-full w-full flex flex-col rounded-lg overflow-hidden border border-gray-700 bg-gray-800">
      {/* Session header */}
      <div className="h-8 flex items-center justify-between px-3 border-b border-gray-700 bg-gray-900">
        <div className="flex items-center gap-2">
          <span
            className="w-2 h-2 rounded-full"
            style={{ backgroundColor: statusConfig.color }}
          />
          <span className="text-sm font-medium text-gray-300">
            Session {session.numericId}
          </span>
          <span
            className="text-xs px-1.5 py-0.5 rounded"
            style={{ backgroundColor: modeConfig.color + '20', color: modeConfig.color }}
          >
            {modeConfig.displayName}
          </span>
        </div>
        <div className="flex items-center gap-2">
          {session.serverUrl && (
            <span className="text-xs text-green-400">{session.serverUrl}</span>
          )}
          <span className="text-xs text-gray-500">{statusConfig.label}</span>
        </div>
      </div>
      {/* Terminal */}
      <div className="flex-1 min-h-0">
        <Terminal sessionId={session.id} />
      </div>
    </div>
  );
}

function App() {
  const { isHydrated } = usePersistence();
  const { sessions, create } = useSessions();
  const [initialized, setInitialized] = useState(false);

  // Set up Tauri event listeners
  useTauriEvents();

  // Initialize default sessions
  useEffect(() => {
    if (!isHydrated || initialized) return;

    const initSessions = async () => {
      // If no sessions exist, create default ones
      if (sessions.length === 0) {
        for (let i = 0; i < DEFAULT_SESSION_COUNT; i++) {
          await create();
        }
      }
      setInitialized(true);
    };

    initSessions();
  }, [isHydrated, initialized, sessions.length, create]);

  // Show loading while hydrating
  if (!isHydrated) {
    return (
      <div className="h-screen w-screen bg-gray-900 flex items-center justify-center">
        <div className="text-gray-400">Loading sessions...</div>
      </div>
    );
  }

  return (
    <div className="h-screen w-screen bg-gray-900 flex flex-col">
      {/* Header */}
      <header className="h-10 bg-gray-800 flex items-center justify-between px-4 border-b border-gray-700">
        <div className="flex items-center">
          <span className="text-white font-medium">Claude Maestro</span>
          <span className="ml-2 text-xs text-gray-400">v2.0.0</span>
        </div>
        <div className="flex items-center gap-4">
          <span className="text-xs text-gray-400">
            {sessions.length} session{sessions.length !== 1 ? 's' : ''}
          </span>
        </div>
      </header>

      {/* Main grid */}
      <main className="flex-1 p-2 min-h-0">
        <div className="h-full grid grid-cols-3 grid-rows-2 gap-2">
          {sessions.slice(0, 6).map((session) => (
            <SessionCard key={session.id} session={session} />
          ))}
        </div>
      </main>
    </div>
  );
}

export default App;
