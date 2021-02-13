#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]    // don't mangle the function name
/// Defines the new entry point.
/// The linker looks for a function named `_start` by default.
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;    // location of the VGA Buffer as
    //                                         // a raw pointer

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;      // character
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;   // color code
    //     }
    // }
    vga_buffer::print_something();

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}