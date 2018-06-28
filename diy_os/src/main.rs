#![feature(panic_implementation)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Called on panic
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}