#![feature(panic_implementation)]
#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;
extern crate volatile;
extern crate spin;

#[macro_use]
mod vga_buffer;

use core::panic::PanicInfo;

// Linker entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

// Catch panics
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
