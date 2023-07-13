#![no_std]
#![cfg_attr(test, no_main)]
#![test_runner(test_runner)] // use our custom test runner
#![feature(custom_test_frameworks)] // enable custom test frameworks
#![reexport_test_harness_main = "test_main"] // rename the test entry point
#![feature(abi_x86_interrupt)]  //This error occurs because the x86-interrupt calling convention is still unstable. To use it anyway, we have to explicitly enable it by adding #![feature(abi_x86_interrupt)]

pub mod serial;
pub mod vga_buffer;
pub mod locks;
pub mod interrupts;
pub mod gdt;

use core::panic::PanicInfo;

#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

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

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}