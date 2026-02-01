import { create } from 'zustand';
import type { AgentState, AgentStatus } from '../types';

interface AgentStateStoreState {
  // Map of session ID to latest agent status
  agentStates: Map<string, AgentStatus>;

  // Actions
  updateAgentState: (
    sessionId: string,
    state: AgentState,
    message: string,
    needsInputPrompt?: string
  ) => void;
  clearAgentState: (sessionId: string) => void;
  clearAllAgentStates: () => void;
}

export const useAgentStateStore = create<AgentStateStoreState>((set, get) => ({
  agentStates: new Map(),

  updateAgentState: (sessionId, state, message, needsInputPrompt) => {
    const agentStates = new Map(get().agentStates);
    agentStates.set(sessionId, {
      sessionId,
      state,
      message,
      needsInputPrompt,
      timestamp: Date.now(),
    });
    set({ agentStates });
  },

  clearAgentState: (sessionId) => {
    const agentStates = new Map(get().agentStates);
    agentStates.delete(sessionId);
    set({ agentStates });
  },

  clearAllAgentStates: () => {
    set({ agentStates: new Map() });
  },
}));

// Selectors
export const selectAgentState =
  (sessionId: string) => (state: AgentStateStoreState) =>
    state.agentStates.get(sessionId);

export const selectAllAgentStates = (state: AgentStateStoreState) =>
  Array.from(state.agentStates.values());

export const selectAgentStatesByState =
  (agentState: AgentState) => (state: AgentStateStoreState) =>
    Array.from(state.agentStates.values()).filter(
      (s) => s.state === agentState
    );
