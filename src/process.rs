// The type for the ID of a running process.
pub type ProcessID = usize;

// The type of the priority of a process.
pub type Priority = usize;

// Type for the state a specific process is currently in.
#[derive(PartialEq)]
pub enum ProcessState {
    Unused,
    Ready,
    Running,
    Blocked,
}

pub struct Process {
    id: ProcessID,
    state: ProcessState,
    priority: Priority,
}

impl Process {
    pub fn is_runnable(&self) -> bool {
        self.state == ProcessState::Ready || self.state == ProcessState::Running
    }
}