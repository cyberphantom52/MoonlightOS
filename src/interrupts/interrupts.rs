use crate::interrupts::{
    idt::InterruptDescriptorTable,
    timer::{timer, PIC_1_OFFSET},
};
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.add_exceptions();
        // idt.add(PIC_1_OFFSET as usize, timer as u64);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
