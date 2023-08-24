use bit_field::BitField;
use x86_64::registers::segmentation::Segment;
use super::exceptions;

const IDT_ENTRIES: usize = 256;

// Reference: https://wiki.osdev.org/Interrupt_Descriptor_Table
#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    entries: [IdtEntry; IDT_ENTRIES],
}

impl InterruptDescriptorTable {
    /// Creates a new IDT filled with non-present entries.
    #[inline]
    pub fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable {
            entries: [IdtEntry::missing(); IDT_ENTRIES],
        }
    }

    #[inline]
    pub fn load(&self) {
        let descriptor = IdtDescriptor {
            size: (core::mem::size_of::<InterruptDescriptorTable>() - 1) as u16,
            offset: self,
        };
        crate::println!("[!] Loading IDT");
        unsafe {
            core::arch::asm!("lidt [{}]", in(reg) &descriptor);
        }
    }


    pub fn add(&mut self, int: usize, handler: u64) {
        self.entries[int].set_handler_addr(handler);
    }
    
    //add exception handlers for various cpu exceptions
    pub fn add_exceptions(&mut self) {
        self.add(0x0, exceptions::div_error as u64);
        self.add(0x6, exceptions::invalid_opcode as u64);
        self.add(0x8, exceptions::double_fault as u64);
        self.add(0xd, exceptions::general_protection_fault as u64);
        self.add(0xe, exceptions::page_fault as u64);
    }
}

#[repr(C, packed)]
pub struct IdtDescriptor {
    size: u16,                               //idt size
    offset: *const InterruptDescriptorTable, //pointer to idt
}

// 128-bits - 16 bytes
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
        let mut entry = IdtEntry {
            offset_lower: 0,
            gdt_selector: 0,
            options: IdtEntryOptions::minimal(),
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
        };
        entry.set_handler_addr(exceptions::generic_handler as u64);
        entry
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
}

/// Represents the options field of an IDT entry.
/// | Bit   | Name	                      | Description
/// | 0-2	| Interrupt Stack Table Index |	0: Don’t switch stacks 1-7: Switch to the n-th stack in the Interrupt Stack Table when this handler is called.
/// | 3-7	| Reserved  
/// | 8	    | 0: Interrupt Gate, 1: Trap Gate	If this bit is 0, interrupts are disabled when this handler is called.
/// | 9-11  | must be one
/// | 12	| must be zero
/// | 13‑14	| Descriptor Privilege Level (DPL)	The minimal privilege level required for calling this handler.
/// | 15	| Present
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
    /// switch to the specified stack before the handler is invoked. This allows kernels to
    /// recover from corrupt stack pointers (e.g., on kernel stack overflow).
    ///
    /// An IST stack is specified by an IST index between 0 and 6 (inclusive). Using the same
    /// stack for multiple interrupts can be dangerous when nested interrupts are possible.
    ///
    /// This function panics if the index is not in the range 0..7.
    ///
    /// ## Safety
    /// This function is unsafe because the caller must ensure that the passed stack index is
    /// valid and not used by other interrupts. Otherwise, memory safety violations are possible.
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
/// Reference: https://wiki.osdev.org/Security#Rings
#[repr(u8)]
pub enum PrivilegeLevel {
    /// Privilege-level 0 (most privilege)
    Ring0 = 0,

    /// Privilege-level 1 (moderate privilege)
    Ring1 = 1,

    /// Privilege-level 2 (moderate privilege)
    Ring2 = 2,

    /// Privilege-level 3 (least privilege)
    Ring3 = 3,
}

impl PrivilegeLevel {
    #[inline]
    pub fn from_u16(value: u16) -> PrivilegeLevel {
        match value {
            0 => PrivilegeLevel::Ring0,
            1 => PrivilegeLevel::Ring1,
            2 => PrivilegeLevel::Ring2,
            3 => PrivilegeLevel::Ring3,
            i => panic!("{} is not a valid privilege level", i),
        }
    }
}
