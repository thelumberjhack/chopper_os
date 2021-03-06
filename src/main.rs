#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(chopper_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use chopper_os::println;

/// Defines the new entry point.
/// The linker looks for a function named `_start` by default.
#[no_mangle]    // don't mangle the function name
pub extern "C" fn _start() -> ! {
    println!("Hello World! {}", "Woohoo!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    chopper_os::test_panic_handler(info)
}