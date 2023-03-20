#![no_std]
#![no_main]
#![feature(panic_info_message)]

use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;
mod config;
use config::NCPU;
use core::sync::atomic::{AtomicBool, Ordering};


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


// core::arch::global_asm!(
//     "
//     .section .text.entry
//     .globl _start
// _start:
//     la sp, stack

//     call rust_main
// spin:
//     j spin
// "
// );

#[no_mangle]
#[link_section = ".bss.stack"]
static mut stack: [u8; 4096 * NCPU] = [0u8; 4096 * NCPU];



static AP_CAN_INIT: AtomicBool = AtomicBool::new(false);


fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}


use sbi::send_ipi;

#[no_mangle]
pub fn rust_main(hartid: usize) -> ! {


    send_ipi(0xd);
    
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
    logging::init();
    println!("Hello, world! {}",hartid);
    println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    unsafe {
        println!(".stack [{:#x}]", &stack as *const _ as usize);
    }

    println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);


    loop{}

    panic!("Shutdown machine!");
}



