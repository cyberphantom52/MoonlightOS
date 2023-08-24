//TIMER INTERRUPT HANDLER
//Used to trigger the cpu scheduler and to context switch

use core::arch::asm;

use pic8259::ChainedPics;

use crate::locks::mutex::Mutex;

//TIMER IRQ
#[naked]
pub extern "C" fn timer() {
    unsafe {
        asm!(
            //disable interrupts
            "cli",
            //save registers
            // "push rbp",
            // "push rdi",
            // "push rsi",
            // "push rdx",
            // "push rcx",
            // "push rbx",
            // "push rax",
            //call c function with esp as argument
            "push rsp",
            "call timer_handler",
            //set esp to return value of c func
            "mov rsp, rax",
            //restore registers
            // "pop rax",
            // "pop rbx",
            // "pop rcx",
            // "pop rdx",
            // "pop rsi",
            // "pop rdi",
            // "pop rbp",
            //re-enable interrupts
            "sti",
            //return irq
            "iretd",
            options(noreturn)
        );
    }
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[no_mangle]
pub extern "C" fn timer_handler() {
    unsafe {
        PICS.lock().notify_end_of_interrupt(PIC_1_OFFSET);
    }
}
