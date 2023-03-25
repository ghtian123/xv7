use spin::Mutex;

use alloc::collections::VecDeque;

use super::*;
use crate::config::{MEMORY_END, PAGE_SIZE};

pub struct Kmm(Mutex<VecDeque<usize>>);

extern "C" {
    fn ekernel();
}

impl Kmm {
    fn new() -> Self {
        let mut free = VecDeque::new();
        let mut p = PGROUNDUP(ekernel as usize);
        while p + PAGE_SIZE <= MEMORY_END {
            free.push_back(p);
            p += PAGE_SIZE;
        }

        Mutex::new(free)
    }

    fn kalloc(&mut self) -> Option<usize> {
        if let Some(addr) = self.0.lock().pop_front() {
            //todo memset

            return Some(addr);
        }
        return None;
    }

    fn kfree(&mut self, addr: usize) {
        if addr % PAGE_SIZE != 0 || addr < ekernel as usize || addr >= MEMORY_END {
            panic!("kfree")
        }

        //todo memset

        self.0.lock().push_front(addr);
    }
}
