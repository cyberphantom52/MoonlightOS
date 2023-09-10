use super::idt::InterruptStackFrame;

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
