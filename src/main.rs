#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

/// This function is called on panic when running.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// This function is called on panic when running tests
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

/// Testable trait used to automatically add print statements to test functions
pub trait Testable {
    fn run(&self) -> ();
}

/// Testable trait implementation for all types `T` that implements the Fn() trait
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[no_mangle]    // don't mangle the function name
/// Defines the new entry point.
/// The linker looks for a function named `_start` by default.
pub extern "C" fn _start() -> ! {
    println!("Hello World! {}", "Woohoo!");
    #[cfg(test)]
    test_main();

    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Exiting qemu after a test by sending the exit code to a port on the IO bus.
/// 0xf4 is defined as the `iobase` of `isa-debug-exit` in `Cargo.toml`
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // Writing to an IO port can generally result in arbitrary behavior
        let mut port = Port::new(0xf4); // 0xf4 is generally unused
        port.write(exit_code as u32);
    }
}
