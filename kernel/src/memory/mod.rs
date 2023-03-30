mod kalloc;
mod kheap;
mod page;
mod page_table;
pub use kheap::*;
pub use page::*;
pub use page_table::*;

pub use kalloc::*;

use crate::config::PAGE_SIZE;

// #define PGROUNDUP(sz)  (((sz)+PGSIZE-1) & ~(PGSIZE-1))
// #define PGROUNDDOWN(a) (((a)) & ~(PGSIZE-1))

pub fn PGROUNDDOWN(sz: usize) -> usize {
    // sz / PAGE_SIZE
    sz & !(PAGE_SIZE - 1)
}

pub fn PGROUNDUP(sz: usize) -> usize {
    // (sz - 1 + PAGE_SIZE) / PAGE_SIZE
    (sz - 1 + PAGE_SIZE) & !(PAGE_SIZE - 1)
}
