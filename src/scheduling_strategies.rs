use crate::{pqueue::ProcessQueue, process::{Priority, Process, ProcessID}};

#[derive(PartialEq)]
pub enum SchedulingStrategies {
    MLFQ,
}

pub struct MLFQ {
    queues: [ProcessQueue; 4],
    timeslices: [u8; 8],
}

impl MLFQ {
    pub fn init(&mut self) -> () {
        for queue in &mut self.queues { queue.init() }

        for timeslice in &mut self.timeslices { *timeslice = 0 }
    }
    
    pub fn reset(&mut self) -> () {
        for queue in &mut self.queues { queue.reset() }

        for timeslice in &mut self.timeslices { *timeslice = 0 }
    }

    pub fn reset_proc(&mut self, pid: ProcessID) {
        self.timeslices[pid] = 0;
    }

    pub fn remove_pid(&mut self, pid: ProcessID) {
        for queue in &mut self.queues { queue.remove_pid(pid); }
    }

    pub fn get_default_timeslice(qid: u8) -> u8 {
        match qid {
            0 => 1,
            1 => 2,
            2 => 4,
            3 => 8,
            _ => 0,
        }
    }

    pub fn map_to_queue(prio: Priority) -> usize {
        3 - ((prio & 0b11000000) >> 6)
    }

    pub fn get_next(&mut self, processes: [Process; 8], current: ProcessID) {//-> ProcessID {
        for queue in &mut self.queues {
            while queue.has_next() {
                let mut pid = queue.peek().unwrap();
                let mut oldhead = queue.head;
            }
        }
    }
}

