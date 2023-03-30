use spin::Mutex;

use alloc::collections::VecDeque;
use core::ptr;
use lazy_static::lazy_static;

use super::*;
use crate::config::{MEMORY_END, PAGE_SIZE};

pub struct KernelMemory(VecDeque<usize>);

extern "C" {
    fn ekernel();
}

impl KernelMemory {
    fn new() -> Self {
        let mut free = VecDeque::new();
        KernelMemory(free)
    }

    fn init(&mut self) {
        let mut p = PGROUNDUP(ekernel as usize);
        while p + PAGE_SIZE <= MEMORY_END {
            p += PAGE_SIZE;
            self.0.push_back(p)
        }
    }

    pub fn kalloc(&mut self) -> Option<usize> {
        if let Some(addr) = self.0.pop_front() {
            // unsafe {
            //     ptr::write_bytes(addr as *mut u8, 0, PAGE_SIZE);
            // }
            return Some(addr);
        }
        return None;
    }

    pub fn kfree(&mut self, addr: usize) {
        if addr % PAGE_SIZE != 0 || addr < ekernel as usize || addr >= MEMORY_END {
            panic!("kfree")
        }
        // unsafe {
        //     ptr::write_bytes(addr as *mut u8, 0, 4096);
        // }
        self.0.push_back(addr)
    }
}

lazy_static! {
    pub static ref KALLOC: Mutex<KernelMemory> = { Mutex::new(KernelMemory::new()) };
}

pub fn kinit() {
    KALLOC.lock().init();
    println!("done")
}
