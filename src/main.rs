#![no_std] //Do not link the Rust Standard Library
#![no_main] //Disables all Rust-level entry points

use core::panic::PanicInfo;

//Using no_mangle to ensure Rust outputs a fn with the name _start.
//Overwriting the OS entry point with our own _start fn.
//extern "C" to tell the compiler to use the C calling convention.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

//Defining our own panic handler.
//Will be called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {}

