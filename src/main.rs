#![no_std] //Do not link the Rust Standard Library
#![no_main] //Disables all Rust-level entry points

use core::panic::PanicInfo;

//Defining our own panic handler.
//Will be called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

//Using no_mangle to ensure Rust outputs a fn with the name _start.
//Overwriting the OS entry point with our own _start fn.
//extern "C" to tell the compiler to use the C calling convention.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Casting raw int 0xb8000 to a raw pointer
    let vga_buffer = 0xb8000 as *mut u8;

    // Iterate over the bytes of the static byte string
    // Enumerate to get a running variable i
    // Offset to write the string byte and the corresponding color byte
    for (i, &byte) in HELLO.iter().enumerate() {
        // Unsafe tells the compiler we are 100% sure the operations are valid
        // Not good to do!
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

fn main() {}

