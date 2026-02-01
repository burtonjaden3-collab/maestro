use parking_lot::RwLock;
use std::collections::HashMap;

use super::managed::{ManagedProcess, ManagedProcessStatus, ProcessSource};

/// Registry tracking all managed processes across sessions
pub struct ProcessRegistry {
    /// Processes indexed by PID
    processes: RwLock<HashMap<u32, ManagedProcess>>,
    /// Session ID -> list of PIDs mapping
    session_pids: RwLock<HashMap<String, Vec<u32>>>,
}

impl ProcessRegistry {
    pub fn new() -> Self {
        Self {
            processes: RwLock::new(HashMap::new()),
            session_pids: RwLock::new(HashMap::new()),
        }
    }

    /// Register a new process
    pub fn register(
        &self,
        session_id: &str,
        pid: u32,
        source: ProcessSource,
        command: &str,
    ) -> ManagedProcess {
        let process = ManagedProcess::new(session_id.to_string(), pid, source, command.to_string());

        {
            let mut processes = self.processes.write();
            processes.insert(pid, process.clone());
        }

        {
            let mut session_pids = self.session_pids.write();
            session_pids
                .entry(session_id.to_string())
                .or_default()
                .push(pid);
        }

        process
    }

    /// Register with process group ID (Unix)
    pub fn register_with_pgid(
        &self,
        session_id: &str,
        pid: u32,
        pgid: u32,
        source: ProcessSource,
        command: &str,
    ) -> ManagedProcess {
        let process = ManagedProcess::new(session_id.to_string(), pid, source, command.to_string())
            .with_pgid(pgid);

        {
            let mut processes = self.processes.write();
            processes.insert(pid, process.clone());
        }

        {
            let mut session_pids = self.session_pids.write();
            session_pids
                .entry(session_id.to_string())
                .or_default()
                .push(pid);
        }

        process
    }

    /// Get a process by PID
    pub fn get(&self, pid: u32) -> Option<ManagedProcess> {
        self.processes.read().get(&pid).cloned()
    }

    /// Get all processes for a session
    pub fn get_session_processes(&self, session_id: &str) -> Vec<ManagedProcess> {
        let session_pids = self.session_pids.read();
        let processes = self.processes.read();

        session_pids
            .get(session_id)
            .map(|pids| {
                pids.iter()
                    .filter_map(|pid| processes.get(pid).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Update process status
    pub fn update_status(&self, pid: u32, status: ManagedProcessStatus) -> Option<ManagedProcess> {
        let mut processes = self.processes.write();
        if let Some(process) = processes.get_mut(&pid) {
            process.status = status;
            Some(process.clone())
        } else {
            None
        }
    }

    /// Update process server info
    pub fn update_server(&self, pid: u32, port: u16, url: &str) -> Option<ManagedProcess> {
        let mut processes = self.processes.write();
        if let Some(process) = processes.get_mut(&pid) {
            process.set_server(port, url.to_string());
            Some(process.clone())
        } else {
            None
        }
    }

    /// Remove a process
    pub fn remove(&self, pid: u32) -> Option<ManagedProcess> {
        let process = {
            let mut processes = self.processes.write();
            processes.remove(&pid)
        };

        if let Some(ref p) = process {
            let mut session_pids = self.session_pids.write();
            if let Some(pids) = session_pids.get_mut(&p.session_id) {
                pids.retain(|&p| p != pid);
            }
        }

        process
    }

    /// Remove all processes for a session
    pub fn remove_session(&self, session_id: &str) -> Vec<ManagedProcess> {
        let pids: Vec<u32> = {
            let mut session_pids = self.session_pids.write();
            session_pids.remove(session_id).unwrap_or_default()
        };

        let mut removed = Vec::new();
        let mut processes = self.processes.write();
        for pid in pids {
            if let Some(p) = processes.remove(&pid) {
                removed.push(p);
            }
        }

        removed
    }

    /// Get count of active processes for a session
    pub fn active_count(&self, session_id: &str) -> usize {
        self.get_session_processes(session_id)
            .iter()
            .filter(|p| {
                matches!(
                    p.status,
                    ManagedProcessStatus::Starting | ManagedProcessStatus::Running
                )
            })
            .count()
    }

    /// Get all PIDs for a session (for cleanup)
    pub fn get_session_pids(&self, session_id: &str) -> Vec<u32> {
        self.session_pids
            .read()
            .get(session_id)
            .cloned()
            .unwrap_or_default()
    }
}

impl Default for ProcessRegistry {
    fn default() -> Self {
        Self::new()
    }
}
