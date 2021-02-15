#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]    // don't mangle the function name
/// Defines the new entry point.
/// The linker looks for a function named `_start` by default.
pub extern "C" fn _start() -> ! {
    println!("Hello World! {}", "Woohoo!");
    panic!("Panicking, haaaaaaaa!!!");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}