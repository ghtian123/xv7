//xv7 内核配置


//cpu 个数
pub const NCPU: usize = 8;
pub const KERNEL_HEAP_SIZE: usize = 0x20_0000;
pub const MEMORY_END: usize = 0x88000000;
pub const PAGE_SIZE: usize = 0x1000;