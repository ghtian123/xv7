#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod config;
mod lang_items;
mod memory;
mod rustsbi;

use crate::config::NCPU;
use core::sync::atomic::{AtomicBool, Ordering};

//使用alloc 数据结构
extern crate alloc;

use crate::memory::{kheap_init};

core::arch::global_asm!(
    "
    .section .text.entry
    .globl _start
_start:

    # a0 == hartid
    # pc == 0x80200000
    # sp == 0x800xxxxx
    # sp = bootstack + (hartid + 1) * 0x10000

    add  t0, a0, 1
    slli t0, t0, 14
    la   sp, stack
    add  sp, sp, t0

    call  rust_main
spin:
    j spin
"
);

#[no_mangle]
#[link_section = ".bss.stack"]
static mut stack: [u8; 4096 * NCPU] = [0u8; 4096 * NCPU];
static SMP_START: AtomicBool = AtomicBool::new(false);

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

pub fn boot_all_harts(hartid: usize) {
    extern "C" {
        fn _start();
    }

    SMP_START.store(true, Ordering::Relaxed);

    for id in (0..NCPU).filter(|i| *i != hartid) {
        // priv: 1 for supervisor; 0 for user;
        let _ = rustsbi::hart_start(id, _start as usize, 1);
    }
}

#[no_mangle]
pub fn rust_main(hartid: usize) -> ! {
    if !SMP_START.load(Ordering::Acquire) {
        extern "C" {
            fn stext();
            fn etext();
            fn srodata();
            fn erodata();
            fn sdata();
            fn edata();
            fn sbss();
            fn ebss();
        }

        clear_bss();
        //初始化rust堆内存，后面就可以使用allco的数据结构
        kheap_init();

        println!("Hello, world! {}", hartid);
        println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        unsafe {
            println!(".stack [{:#x}]", &stack as *const _ as usize);
        }

        println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

        boot_all_harts(hartid);

        loop {}
    } else {
        println!("Hello, world! {}", hartid);

        loop {}
    }

    panic!("shut down!")
}
