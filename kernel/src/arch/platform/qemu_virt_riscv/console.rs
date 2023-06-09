use core::fmt::{self, Write};
use sbi_rt::legacy::console_putchar;
use spin::Mutex;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    static LOCK: Mutex<()> = Mutex::new(());
    let _guard = LOCK.lock();
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch::platform::qemu_virt_riscv::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch::platform::qemu_virt_riscv::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
