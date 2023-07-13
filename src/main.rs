#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(moonlight_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use moonlight_os::println;

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    println!("Moonlight {}", "OS");

    moonlight_os::init();

    //Below line triggers a double fault exception
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };

    //Below line triggers a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();
    // println!("It did not crash!");
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
    moonlight_os::test_panic_handler(info)
}
