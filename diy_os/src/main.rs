#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(panic_implementation)]
#![no_std]
#![cfg_attr(not(test), no_main)]

#[macro_use]
extern crate lazy_static;
extern crate volatile;
extern crate spin;
#[cfg(test)]
extern crate std;
#[cfg(test)]
extern crate array_init;

#[macro_use]
mod vga_buffer;

use core::panic::PanicInfo;

// Linker entry point
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

// Catch panics
#[cfg(not(test))] 
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
