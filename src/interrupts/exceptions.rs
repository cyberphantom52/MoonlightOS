use super::idt::InterruptStackFrame;
/*
 C Calling Convention:
    - The first six integer arguments are passed in registers `rdi`, `rsi`, `rdx`, `rcx`, `r8`, `r9`
    - Additional arguments are passed on the stack
    - Results are returned in rax and rdx

*/

//TODO: use macros to save/restore scratch registers
macro_rules! handler_with_err {
    ($name:ident) => {{
        #[naked]
        pub extern "C" fn wrapper() -> ! {
            unsafe {
                core::arch::asm!(
                    // Save scratch registers
                    "push rax",
                    "push rcx",
                    "push rdx",
                    "push rsi",
                    "push rdi",
                    "push r8",
                    "push r9",
                    "push r10",
                    "push r11",

                    "mov rsi, [rsp + 9*8]", // load error code into rsi
                    "mov rdi, rsp",
                    "add rdi, 10*8", // calculate exception stack frame pointer. 10 because error code is also pushed to stack along with scratch registers
                    "sub rsp, 8", // align stack pointer to 16 byte boundary
                    "call {}",
                    "add rsp, 8", // restore stack pointer

                    // Restore scratch registers
                    "pop r11",
                    "pop r10",
                    "pop r9",
                    "pop r8",
                    "pop rdi",
                    "pop rsi",
                    "pop rdx",
                    "pop rcx",
                    "pop rax",
                    "add rsp, 8", // remove error code from stack
                    "iretq",
                    sym $name,
                    options(noreturn)
                );
            }
        }
        wrapper as u64
    }};
}

macro_rules! handler {
    ($name:ident) => {{
        #[naked]
        pub extern "C" fn wrapper() -> ! {
            unsafe {
                core::arch::asm!(
                    // Save scratch registers
                    "push rax",
                    "push rcx",
                    "push rdx",
                    "push rsi",
                    "push rdi",
                    "push r8",
                    "push r9",
                    "push r10",
                    "push r11",

                    "mov rdi, rsp",
                    "add rdi, 9*8", // calculate exception stack frame pointer
                    "call {}",

                    // Restore scratch registers
                    "pop r11",
                    "pop r10",
                    "pop r9",
                    "pop r8",
                    "pop rdi",
                    "pop rsi",
                    "pop rdx",
                    "pop rcx",
                    "pop rax",
                    "iretq",
                    sym $name,
                    options(noreturn)
                );
            }
        }
        wrapper as u64
    }};
}
pub(crate) use handler;
pub(crate) use handler_with_err;

// Reference: https://os.phil-opp.com/cpu-exceptions/#the-interrupt-calling-convention
#[no_mangle]
pub extern "C" fn div_error_handler(stack_frame: &InterruptStackFrame) {
    panic!("EXCEPTION: DIVISION ERROR\n{:#?}", stack_frame);
}

#[no_mangle]
pub extern "C" fn invalid_opcode_handler(stack_frame: &InterruptStackFrame) {
    panic!("EXCEPTION: INVALID OPCODE\n{:#?}", stack_frame);
}

#[no_mangle]
pub extern "C" fn breakpoint_handler(stack_frame: &InterruptStackFrame) {
    crate::println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[no_mangle]
pub extern "C" fn double_fault_handler(stack_frame: &InterruptStackFrame, _error_code: u64) {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[no_mangle]
pub extern "C" fn general_protection_fault_handler(
    stack_frame: &InterruptStackFrame,
    _error_code: u64,
) {
    panic!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", stack_frame);
}

#[no_mangle]
pub extern "C" fn page_fault_handler(stack_frame: &InterruptStackFrame, error_code: u64) {
    crate::print!("EXCEPTION: PAGE FAULT occured\nError Code: {:?}\n{:#?}", error_code, stack_frame);
}

#[no_mangle]
pub extern "C" fn generic_handler(stack_frame: &InterruptStackFrame) {
    panic!("EXCEPTION: GENERIC\n{:#?}", stack_frame);
}
