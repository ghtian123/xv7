#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(stdsimd)]
#[macro_use]
mod arch;
mod config;
mod lang_items;
mod memory;

use crate::config::NCPU;
pub use arch::platform::qemu_virt_riscv::console::*;
use core::sync::atomic::{AtomicBool, Ordering};

//使用alloc 数据结构
extern crate alloc;

use crate::memory::kheap_init;

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        "add  t0, a0, 1",
        "slli t0, t0, 12",
        "la   sp, {stack}",
        "add  sp, sp, t0",
        "call  {main}",

        stack      =   sym stack,
        main       =   sym rust_main,
        options(noreturn),
    )
}

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

    SMP_START.store(true, Ordering::SeqCst);

    println!("I am cpu id {}", hartid);
    for id in (0..NCPU).filter(|i| *i != hartid) {
        // priv: 1 for supervisor; 0 for user;
        println!("Starting cpu id {}", id);
        let x = sbi_rt::hart_start(id, _start as usize, 1);
        println!("Starting ret {}--{:?}", id, x);
    }
}

#[no_mangle]
pub fn rust_main(hartid: usize) -> ! {
    if !SMP_START.load(Ordering::SeqCst) {
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
