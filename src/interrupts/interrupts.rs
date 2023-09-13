use crate::{
    instructions::{disable_interrupts, enable_interrupts, interrupts_enabled},
    interrupts::idt::InterruptDescriptorTable,
    locks::mutex::Mutex,
    println,
    shell::shell::SHELL,
    pic::ChainedPics,
};
use lazy_static::lazy_static;
use super::idt::InterruptStackFrame;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.add_exceptions();
        idt.add(PIC_1_OFFSET as usize, timer_interrupt_handler as u64);
        idt.add(33, keyboard_interrupt_handler as u64);
        idt
    };
}

pub fn init_idt() {
    println!("[!] Loading IDT");
    println!("    [+] Setting up exceptions");
    println!("    [+] Setting up PIC interrupts");
    println!("    [+] Setting up keyboard interrupts");
    IDT.load();
    println!("    [+] Done")
}

// Ref: https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html
pub fn without_interrupts<F>(f: F)
where
    F: FnOnce(),
{
    let status = interrupts_enabled();

    if status {
        disable_interrupts();
    }

    f();

    if status {
        enable_interrupts();
    }
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

extern "x86-interrupt" fn timer_interrupt_handler(_: &mut InterruptStackFrame) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(PIC_1_OFFSET);
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();

    let scancode: u8;
    unsafe {
        core::arch::asm!("in al, dx", out("al") scancode, in("dx") 0x60 as u16);
    }

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    let mut shell = SHELL.lock();
                    // Backspace
                    if character == '\n' {
                        shell.enter();
                    } else if character == '\u{8}' {
                        shell.backspace();
                    } else {
                        shell.add(character);
                    }
                    drop(shell);
                }

                _ => {}
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(33);
    }
}
