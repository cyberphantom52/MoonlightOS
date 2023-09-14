use super::idt::InterruptStackFrame;
/*
 C Calling Convention:
    - The first six integer arguments are passed in registers `rdi`, `rsi`, `rdx`, `rcx`, `r8`, `r9`
    - Additional arguments are passed on the stack
    - Results are returned in rax and rdx

*/

macro_rules! handler_with_err {
    ($name:ident) => {{
        #[naked]
        pub extern "C" fn wrapper() -> ! {
            unsafe {
                core::arch::asm!(
                    "pop rsi", // pop error code into rsi
                    "mov rdi, rsp",
                    "sub rsp, 8", // align stack pointer to 16 byte boundary
                    "call {}",
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
                    "mov rdi, rsp",
                    "sub rsp, 8", // align stack pointer to 16 byte boundary
                    "call {}",
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

//CPU EXCEPTIONS HANDLERS
// Reference: https://os.phil-opp.com/cpu-exceptions/#the-interrupt-calling-convention
pub extern "x86-interrupt" fn div_error_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: DIVISION ERROR\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: INVALID OPCODE\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: PAGE FAULT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn generic_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: GENERIC\n{:#?}", stack_frame);
}
