import { useEffect, useRef, useCallback } from 'react';
import { Terminal as XTerm } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { WebglAddon } from '@xterm/addon-webgl';
import { ptyService } from '../services/ptyService';
import { useSession } from '../hooks';
import '@xterm/xterm/css/xterm.css';

interface TerminalProps {
  sessionId: string;
  onReady?: () => void;
}

export function Terminal({ sessionId, onReady }: TerminalProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const terminalRef = useRef<XTerm | null>(null);
  const fitAddonRef = useRef<FitAddon | null>(null);
  const { session, spawnPty } = useSession(sessionId);

  const handleResize = useCallback(() => {
    if (fitAddonRef.current && terminalRef.current) {
      fitAddonRef.current.fit();
    }
  }, []);

  useEffect(() => {
    if (!containerRef.current || terminalRef.current) return;

    const terminal = new XTerm({
      fontFamily: "Menlo, Monaco, 'Courier New', monospace",
      fontSize: 13,
      theme: {
        background: '#1e1e2e',
        foreground: '#cdd6f4',
        cursor: '#f5e0dc',
        cursorAccent: '#1e1e2e',
        black: '#45475a',
        red: '#f38ba8',
        green: '#a6e3a1',
        yellow: '#f9e2af',
        blue: '#89b4fa',
        magenta: '#f5c2e7',
        cyan: '#94e2d5',
        white: '#bac2de',
      },
      cursorBlink: true,
      allowProposedApi: true,
    });

    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.open(containerRef.current);

    try {
      const webglAddon = new WebglAddon();
      terminal.loadAddon(webglAddon);
    } catch (e) {
      console.warn('WebGL addon failed, using canvas renderer');
    }

    fitAddon.fit();
    fitAddonRef.current = fitAddon;
    terminalRef.current = terminal;

    // Spawn PTY on backend with session integration
    const workingDir = session?.workingDirectory ?? undefined;
    spawnPty(workingDir).catch(console.error);

    // Handle user input -> send to PTY
    terminal.onData((data) => {
      ptyService.write(sessionId, data).catch(console.error);
    });

    // Handle resize
    terminal.onResize(({ cols, rows }) => {
      ptyService.resize(sessionId, cols, rows).catch(console.error);
    });

    // Listen for PTY output from backend
    let unlistenOutput: (() => void) | null = null;
    ptyService.onOutput(sessionId, (data) => {
      terminal.write(data);
    }).then((unlisten) => {
      unlistenOutput = unlisten;
    });

    // Handle window resize
    window.addEventListener('resize', handleResize);
    const resizeObserver = new ResizeObserver(handleResize);
    resizeObserver.observe(containerRef.current);

    // Notify ready
    onReady?.();

    return () => {
      unlistenOutput?.();
      window.removeEventListener('resize', handleResize);
      resizeObserver.disconnect();
      terminal.dispose();
      terminalRef.current = null;
      fitAddonRef.current = null;
    };
  }, [sessionId, session?.workingDirectory, spawnPty, handleResize, onReady]);

  return <div ref={containerRef} className="h-full w-full" />;
}
