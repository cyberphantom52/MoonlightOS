use super::exceptions;
use bit_field::BitField;
use x86_64::registers::segmentation::Segment;

const IDT_ENTRIES: usize = 256;

// Reference: https://wiki.osdev.org/Interrupt_Descriptor_Table
#[repr(C, align(16))]
pub struct InterruptDescriptorTable {
    pub entries: [IdtEntry; IDT_ENTRIES],
}

// Reference: https://wiki.osdev.org/Interrupt_Descriptor_Table#IDTR
#[repr(C, packed)]
pub struct IdtDescriptor {
    size: u16,                               //idt size
    offset: *const InterruptDescriptorTable, //pointer to idt
}

impl InterruptDescriptorTable {
    /// Creates a new IDT filled with non-present entries.
    #[inline]
    pub fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable {
            entries: [IdtEntry::missing(); IDT_ENTRIES],
        }
    }

    // Need to ensure IDT reference has static lifetime
    // Reference: https://os.phil-opp.com/catching-exceptions/#safety
    #[inline]
    pub fn load(&'static self) {
        let descriptor = IdtDescriptor {
            size: (core::mem::size_of::<Self>() - 1) as u16,
            offset: self,
        };
        unsafe {
            core::arch::asm!("lidt [{}]", in(reg) &descriptor);
        }
    }

    pub fn add(mut self, int: usize, handler: u64) -> InterruptDescriptorTable {
        self.entries[int].set_handler_addr(handler);
        self
    }

    //add exception handlers for various cpu exceptions
    pub fn add_exceptions(self) -> InterruptDescriptorTable {
        self.add(0x0, exceptions::div_error_handler as u64)
            .add(0x3, exceptions::breakpoint_handler as u64)
            .add(0x6, exceptions::invalid_opcode_handler as u64)
            .add(0x8, exceptions::double_fault_handler as u64)
            .add(0xd, exceptions::general_protection_fault_handler as u64)
            .add(0xe, exceptions::page_fault_handler as u64)
    }
}

/// Reference: https://wiki.osdev.org/Interrupt_Descriptor_Table#Gate_Descriptor_2
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdtEntry {
    offset_lower: u16,
    gdt_selector: u16,
    options: IdtEntryOptions,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    /// Creates a non-present IDT entry (but sets the must-be-one bits).
    #[inline]
    pub fn missing() -> Self {
        let addr = exceptions::generic_handler as u64;
        IdtEntry {
            offset_lower: addr as u16,
            gdt_selector: 0,
            options: IdtEntryOptions::minimal(),
            offset_middle: (addr >> 16) as u16,
            offset_high: (addr >> 32) as u32,
            reserved: 0,
        }
    }

    #[inline]
    fn set_handler_addr(&mut self, addr: u64) {
        use x86_64::instructions::segmentation;

        self.offset_lower = addr as u16;
        self.offset_middle = (addr >> 16) as u16;
        self.offset_high = (addr >> 32) as u32;

        self.gdt_selector = segmentation::CS::get_reg().0;

        self.options.set_present(true);
    }

    #[inline]
    pub unsafe fn set_stack_index(&mut self, index: u16) {
        self.options.set_stack_index(index);
    }
}

/// Represents the options field of an IDT entry.
/// Structure:
/// bit 0-2: IST index,
/// bit 3-7: reserved,
/// bit 8: 0 - interrupt gate, 1 - trap gate,
/// bit 9-11: must be 1,
/// bit 12: must be 0,
/// bit 13-14: Descriptor Privilege Level (DPL),
/// bit 15: present bit
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct IdtEntryOptions(u16);

impl IdtEntryOptions {
    /// Creates a minimal options field with all the must-be-one bits set.
    #[inline]
    const fn minimal() -> Self {
        IdtEntryOptions(0b1110_0000_0000)
    }

    /// Set or reset the preset bit.
    #[inline]
    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }

    /// Let the CPU disable hardware interrupts when the handler is invoked. By default,
    /// interrupts are disabled on handler invocation.
    #[inline]
    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }

    /// Assigns a Interrupt Stack Table (IST) stack to this handler. The CPU will then always
    /// switch to the specified stack before the handler is invoked.
    /// 
    /// An IST stack is specified by an IST index between 0 and 6 (inclusive).
    #[inline]
    pub unsafe fn set_stack_index(&mut self, index: u16) -> &mut Self {
        // The hardware IST index starts at 1, but our software IST index
        // starts at 0. Therefore we need to add 1 here.
        self.0.set_bits(0..=2, index + 1);
        self
    }

    #[inline]
    pub fn set_privilege_level(&mut self, dpl: PrivilegeLevel) -> &mut Self {
        self.0.set_bits(13..=14, dpl as u16);
        self
    }
}

/// Represents a protection ring level.
/// 
/// Reference: https://wiki.osdev.org/Security#Rings
#[repr(u8)]
pub enum PrivilegeLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

/// Represents the interrupt stack frame pushed by the CPU on interrupt or exception entry.
#[derive(Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
    /// This value points to the instruction that should be executed when the interrupt
    /// handler returns. For most interrupts, this value points to the instruction immediately
    /// following the last executed instruction. However, for some exceptions (e.g., page faults),
    /// this value points to the faulting instruction, so that the instruction is restarted on
    /// return.
    pub instruction_pointer: u64,
    /// The code segment selector, padded with zeros.
    pub code_segment: u64,
    /// The flags register before the interrupt handler was invoked.
    pub cpu_flags: u64,
    /// The stack pointer at the time of the interrupt.
    pub stack_pointer: u64,
    /// The stack segment descriptor at the time of the interrupt (often zero in 64-bit mode).
    pub stack_segment: u64,
}
