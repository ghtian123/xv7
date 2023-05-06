use spin::Mutex;

use alloc::vec::Vec;

use super::PhysAddr;
use super::*;
use crate::{
    config::{MEMORY_END, PAGE_SIZE},
    println,
};
use core::ptr::write_bytes;

extern "C" {
    fn ekernel();
}

trait FrameAllocator {
    fn init(start: usize, end: usize) -> Self;
    fn kalloc(&mut self) -> Option<PhysAddr>;
    fn kfree(&mut self, phy: PhysAddr);
}

pub struct StackFrameAllocator {
    mem: Vec<PhysAddr>,
}

impl FrameAllocator for StackFrameAllocator {
    fn init(start: usize, end: usize) -> Self {
        let mut mem = Vec::new();
        let s = align_up(start);
        let e = align_down(end);

        while s <= e {
            mem.push(s.into());
            s += PAGE_SIZE;
        }
        Self { mem: mem }
    }

    fn kalloc(&mut self) -> Option<PhysAddr> {
        match self.mem.pop() {
            Some(phy) => {
                unsafe { write_bytes(phy.0 as *mut u8, 0, PAGE_SIZE) }
                Some(phy)
            }
            None => None,
        }
    }

    fn kfree(&mut self, phy: PhysAddr) {
        unsafe { write_bytes(phy.0 as *mut u8, 0, PAGE_SIZE) }
        self.mem.push(phy)
    }
}
