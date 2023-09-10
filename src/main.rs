#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(moonlight_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use moonlight_os::println;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::{structures::paging::Page, VirtAddr};
use moonlight_os::memory::BootInfoFrameAllocator;
use moonlight_os::memory;
use moonlight_os::shell::shell::SHELL;

entry_point!(kernel_main);

#[no_mangle] // don't mangle the name of this function
fn kernel_main(boot_info: &'static BootInfo) -> !{


    println!("Moonlight OS{}", "!");
    moonlight_os::init();

    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
     
    #[cfg(test)]
    test_main();
    println!("It did not crash");
    SHELL.lock().init();
    moonlight_os::hlt_loop();
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
    println!("{}", info);
    moonlight_os::hlt_loop();
}
