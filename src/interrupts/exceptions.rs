
use crate::println;
use core::arch::asm;

//CPU EXCEPTIONS HANDLERS

//handle excpetion based on interrupt number
#[no_mangle]
pub extern "C" fn exception_handler(int: u64, eip: u64, cs: u64, eflags: u64) {
    match int {
        0x00 => {
            println!("DIVISION ERROR!");
        }
        0x06 => {
            println!("INVALID OPCODE!");
        }
        0x08 => {
            println!("DOUBLE FAULT!");
        }
        0x0D => {
            println!("GENERAL PROTECTION FAULT!");
        }
        0x0E => {
            println!("PAGE FAULT!");
        }
        0xFF => {
            println!("EXCEPTION!");
        }
        0x20 => {
            println!("TIMER INTERRUPT!");
        }
        _ => {
            println!("EXCEPTION! not handled {}", int);
        }
    }
    println!("EIP: {:X}, CS: {:X}, EFLAGS: {:b}", eip, cs, eflags);

    loop {}
}

#[naked]
pub extern "C" fn div_error() {
    unsafe {
        asm!(
            "push 0x00",
            "call exception_handler",
            "add rsp, 4",
            "iretd",
            options(noreturn)
        );
    }
}

#[naked]
pub extern "C" fn invalid_opcode() {
    unsafe {
        asm!(
            "push 0x06",
            "call exception_handler",
            "add rsp, 4",
            "iretd",
            options(noreturn)
        );
    }
}

#[naked]
pub extern "C" fn double_fault() {
    unsafe {
        asm!(
            "push 0x08",
            "call exception_handler",
            "add rsp, 4",
            "iretd",
            options(noreturn)
        );
    }
}

#[naked]
pub extern "C" fn general_protection_fault() {
    unsafe {
        asm!(
            "push 0x0d",
            "call exception_handler",
            "add rsp, 4",
            "iretd",
            options(noreturn)
        );
    }
}

#[naked]
pub extern "C" fn page_fault() {
    unsafe {
        asm!(
            "push 0x0e",
            "call exception_handler",
            "add rsp, 4",
            "iretd",
            options(noreturn)
        );
    }
}

#[naked]
pub extern "C" fn generic_handler() {
    unsafe {
        asm!(
            "push 0xff",
            "call exception_handler",
            "add rsp, 4",
            "iretd",
            options(noreturn)
        );
    }
}
