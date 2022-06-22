use crate::process::*;

pub struct ProcessQueue {
    pub data: [ProcessID; 8],
    pub size: usize,
    pub length: usize,
    pub head: usize,
    pub tail: usize,
}

impl ProcessQueue {
    pub fn init(&mut self) -> () {
        self.size = 8;
        self.length = 0;
        self.head = 0;
        self.tail = 0;
    }

    pub fn reset(&mut self) -> () {
        self.length = 0;
        self.head = 0;
        self.tail = 0;
    }

    pub fn has_next(&self) -> bool {
        self.length != 0
    }

    pub fn peek(&self) -> Result<ProcessID, String> {
        if self.has_next() {
            Ok(self.data[self.tail])
        } else {
            Err(String::from("Error: Queue is Empty"))
        }
    }

    pub fn drop(&mut self) -> Result<(),String> {
        if self.length > 0{
            self.tail = self.tail + 1 % self.size;
            self.length -= 1;
            Ok(())
        } else {
            Err(String::from("Erorr: Queue is Empty"))
        }
    }

    pub fn put(&mut self, pid: ProcessID) -> Result<ProcessID, String> {
        if self.length < self.size {
            self.data[self.head] = pid;
            self.head = self.head + 1 % self.size;
            Ok(pid)
        } else {
            Err(String::from("Error: Queue is Full"))
        }
    }

    pub fn get(&mut self) -> Result<ProcessID, String> {
        match self.peek() {
            Ok(pid) => {
                match self.drop() {
                    Ok(_) => Ok(pid),
                    Err(e) => Err(e)
                }
                
            }
            Err(e) => Err(e)
        }
    }

    //TODO handle Result
    pub fn remove_pid(&mut self, pid: ProcessID) -> () { //Result<(), String> {
        if self.has_next() {
            let oldhead = self.head.clone();
            let mut i = self.tail.clone();
            while i != oldhead {
                if self.peek().unwrap() == pid {
                    self.drop().unwrap();
                } else {
                    self.put(self.peek().unwrap()).unwrap();
                    self.drop().unwrap();
                }
                i = (i + 1) % self.size;
            }
        }
    }
}