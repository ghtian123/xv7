use super::{PageTableEntry, KALLOC};

#[derive(Debug)]
#[repr(transparent)]
pub struct PageTable {
    root: usize,
}

impl PageTable {
    // Return the address of the PTE in page table pagetable
    // that corresponds to virtual address va.  If alloc!=0,
    // create any required page-table pages.
    //
    // The risc-v Sv39 scheme has three levels of page-table
    // pages. A page-table page contains 512 64-bit PTEs.
    // A 64-bit virtual address is split into five fields:
    //   39..63 -- must be zero.
    //   30..38 -- 9 bits of level-2 index.
    //   21..29 -- 9 bits of level-1 index.
    //   12..20 -- 9 bits of level-0 index.
    //    0..11 -- 12 bits of byte offset within the page.
    pub fn walk(&mut self, va: usize, alloc: usize) -> PageTableEntry {
        todo!()
    }

    // create an empty user page table.
    // returns 0 if out of memory.
    pub fn uvmcreate() -> Option<Self> {
        //todo unwrap
        if let Some(addr) = KALLOC.lock().kalloc() {
            return Some(PageTable { root: addr });
        }

        None
    }
}
