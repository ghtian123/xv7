use spin::Mutex;

use alloc::vec::Vec;

use super::PhysAddr;
use super::*;
use crate::{
    config::{MEMORY_END, PAGE_SIZE},
    println,
};
use core::ptr::write_bytes;
use lazy_static::*;

pub trait FrameAllocator {
    fn init(&mut self, start: usize, end: usize);
    fn kalloc(&mut self) -> Option<FrameTracker>;
    fn kfree(&mut self, phy: PhysAddr);
}

pub struct StackFrameAllocator {
    mem: Vec<PhysAddr>,
}

impl StackFrameAllocator {
    fn new() -> Self {
        Self { mem: Vec::new() }
    }
}

impl FrameAllocator for StackFrameAllocator {
    fn init(&mut self, start: usize, end: usize) {
        let mut s = align_up(start);
        let e = align_down(end);

        while s < e {
            self.mem.push(s.into());
            s += PAGE_SIZE;
        }
    }

    fn kalloc(&mut self) -> Option<FrameTracker> {
        match self.mem.pop() {
            Some(phy) => {
                unsafe { write_bytes(phy.0 as *mut u8, 0, PAGE_SIZE) }
                Some(phy.into())
            }
            None => None,
        }
    }

    fn kfree(&mut self, phy: PhysAddr) {
        unsafe { write_bytes(phy.0 as *mut u8, 0, PAGE_SIZE) }
        self.mem.push(phy)
    }
}

pub fn kalloc_init() {
    extern "C" {
        fn ekernel();
    }
    FRAME_ALLOCATOR
        .lock()
        .init(ekernel as usize, MEMORY_END as usize)
}

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<StackFrameAllocator> =
        Mutex::new(StackFrameAllocator::new());
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct FrameTracker(PhysAddr);

impl Drop for FrameTracker {
    fn drop(&mut self) {
        FRAME_ALLOCATOR.lock().kfree(self.0)
    }
}

impl From<PhysAddr> for FrameTracker {
    fn from(value: PhysAddr) -> Self {
        Self(value)
    }
}

#[allow(unused)]
pub fn test_alloc() {
    let r = FRAME_ALLOCATOR.lock().kalloc().unwrap();

    println!("{:?}", r);
    drop(r);
    let r = FRAME_ALLOCATOR.lock().kalloc();

    println!("{:?}", r);

    let r = FRAME_ALLOCATOR.lock().kalloc();

    println!("{:?}", r);
}
