use crate::config::PAGE_SIZE;

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

impl From<usize> for PhysAddr {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[inline]
pub fn align_down(sz: usize) -> usize {
    sz & !(PAGE_SIZE - 1)
}

#[inline]
pub fn align_up(sz: usize) -> usize {
    (sz - 1 + PAGE_SIZE) & !(PAGE_SIZE - 1)
}
