use super::{FrameAllocator, FrameTracker, FRAME_ALLOCATOR};
use alloc::vec::Vec;
use bitflags::*;

bitflags! {
    /// page table entry flags
    #[derive(PartialEq)]
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

pub struct PageTable {
    root: FrameTracker,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    fn new() -> Option<Self> {
        match FRAME_ALLOCATOR.lock().kalloc() {
            Some(frame) => Some(Self {
                root: frame,
                frames: Vec::new(),
            }),
            None => None,
        }
    }
    fn from_token(token:usize)->Self{
        todo!()
    }


}



#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry(usize);

impl PageTableEntry {
    fn new() -> Self {
        todo!()
    }

    pub fn empty() -> Self {
        Self(0)
    }
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.0 as u8).unwrap()
    }
    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }
    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }
    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
}
