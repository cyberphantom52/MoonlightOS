use crate::println;

use bit_field::BitField;
use volatile::Volatile;
use core::{arch::asm, marker::PhantomData};
use x86_64::{registers::segmentation::Segment, VirtAddr};

// Reference: https://wiki.osdev.org/Interrupt_Descriptor_Table
#[derive(Clone)]
#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    pub divide_error: IdtEntry<HandlerFunc>,
    pub debug: IdtEntry<HandlerFunc>,
    pub non_maskable_interrupt: IdtEntry<HandlerFunc>,
    pub breakpoint: IdtEntry<HandlerFunc>,
    pub overflow: IdtEntry<HandlerFunc>,
    pub bound_range_exceeded: IdtEntry<HandlerFunc>,
    pub invalid_opcode: IdtEntry<HandlerFunc>,
    pub device_not_available: IdtEntry<HandlerFunc>,
    pub double_fault: IdtEntry<DivergingHandlerFuncWithErrCode>,
    coprocessor_segment_overrun: IdtEntry<HandlerFunc>,
    pub invalid_tss: IdtEntry<HandlerFuncWithErrCode>,
    pub segment_not_present: IdtEntry<HandlerFuncWithErrCode>,
    pub stack_segment_fault: IdtEntry<HandlerFuncWithErrCode>,
    pub general_protection_fault: IdtEntry<HandlerFuncWithErrCode>,
    pub page_fault: IdtEntry<PageFaultHandlerFunc>,
    reserved_1: IdtEntry<HandlerFunc>,
    pub x87_floating_point: IdtEntry<HandlerFunc>,
    pub alignment_check: IdtEntry<HandlerFuncWithErrCode>,
    pub machine_check: IdtEntry<DivergingHandlerFunc>,
    pub simd_floating_point: IdtEntry<HandlerFunc>,
    pub virtualization: IdtEntry<HandlerFunc>,
    reserved_2: [IdtEntry<HandlerFunc>; 9],
    pub security_exception: IdtEntry<HandlerFuncWithErrCode>,
    reserved_3: IdtEntry<HandlerFunc>,
    interrupts: [IdtEntry<HandlerFunc>; 256 - 32],
}

impl InterruptDescriptorTable {
    /// Creates a new IDT filled with non-present entries.
    #[inline]
    pub const fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable {
            divide_error: IdtEntry::missing(),
            debug: IdtEntry::missing(),
            non_maskable_interrupt: IdtEntry::missing(),
            breakpoint: IdtEntry::missing(),
            overflow: IdtEntry::missing(),
            bound_range_exceeded: IdtEntry::missing(),
            invalid_opcode: IdtEntry::missing(),
            device_not_available: IdtEntry::missing(),
            double_fault: IdtEntry::missing(),
            coprocessor_segment_overrun: IdtEntry::missing(),
            invalid_tss: IdtEntry::missing(),
            segment_not_present: IdtEntry::missing(),
            stack_segment_fault: IdtEntry::missing(),
            general_protection_fault: IdtEntry::missing(),
            page_fault: IdtEntry::missing(),
            reserved_1: IdtEntry::missing(),
            x87_floating_point: IdtEntry::missing(),
            alignment_check: IdtEntry::missing(),
            machine_check: IdtEntry::missing(),
            simd_floating_point: IdtEntry::missing(),
            virtualization: IdtEntry::missing(),
            reserved_2: [IdtEntry::missing(); 9],
            security_exception: IdtEntry::missing(),
            reserved_3: IdtEntry::missing(),
            interrupts: [IdtEntry::missing(); 256 - 32],
        }
    }
    // #[inline]
    // pub fn load(&self) {
    //     let descriptor = IdtDescriptor {
    //         size: (core::mem::size_of::<InterruptDescriptorTable>() - 1) as u16,
    //         offset: self,
    //     };
    //     println!("Loading IDT");
    //     unsafe { asm!("lidt [{0:e}]", in(reg) &descriptor) }
    // }

    #[inline]
    pub fn load(&'static self) {
        unsafe { self.load_unsafe() }
    }

    #[inline]
    pub unsafe fn load_unsafe(&self) {
        use x86_64::instructions::tables::lidt;
        unsafe {
            lidt(&self.pointer());
        }
    }

    fn pointer(&self) -> x86_64::structures::DescriptorTablePointer {
        use core::mem::size_of;
        x86_64::structures::DescriptorTablePointer {
            base: VirtAddr::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        }
    }
}

impl core::ops::Index<usize> for InterruptDescriptorTable {
    type Output = IdtEntry<HandlerFunc>;

    /// Returns the IDT entry with the specified index.
    ///
    /// Panics if index is outside the IDT (i.e. greater than 255) or if the entry is an
    /// exception that pushes an error code (use the struct fields for accessing these entries).
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.divide_error,
            1 => &self.debug,
            2 => &self.non_maskable_interrupt,
            3 => &self.breakpoint,
            4 => &self.overflow,
            5 => &self.bound_range_exceeded,
            6 => &self.invalid_opcode,
            7 => &self.device_not_available,
            9 => &self.coprocessor_segment_overrun,
            16 => &self.x87_floating_point,
            19 => &self.simd_floating_point,
            20 => &self.virtualization,
            i @ 32..=255 => &self.interrupts[i - 32],
            i @ 15 | i @ 31 | i @ 21..=29 => panic!("entry {} is reserved", i),
            i @ 8 | i @ 10..=14 | i @ 17 | i @ 30 => {
                panic!("entry {} is an exception with error code", i)
            }
            i @ 18 => panic!("entry {} is an diverging exception (must not return)", i),
            i => panic!("no entry with index {}", i),
        }
    }
}

impl core::ops::IndexMut<usize> for InterruptDescriptorTable {
    /// Returns a mutable reference to the IDT entry with the specified index.
    ///
    /// Panics if index is outside the IDT (i.e. greater than 255) or if the entry is an
    /// exception that pushes an error code (use the struct fields for accessing these entries).
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.divide_error,
            1 => &mut self.debug,
            2 => &mut self.non_maskable_interrupt,
            3 => &mut self.breakpoint,
            4 => &mut self.overflow,
            5 => &mut self.bound_range_exceeded,
            6 => &mut self.invalid_opcode,
            7 => &mut self.device_not_available,
            9 => &mut self.coprocessor_segment_overrun,
            16 => &mut self.x87_floating_point,
            19 => &mut self.simd_floating_point,
            20 => &mut self.virtualization,
            i @ 32..=255 => &mut self.interrupts[i - 32],
            i @ 15 | i @ 31 | i @ 21..=28 => panic!("entry {} is reserved", i),
            i @ 8 | i @ 10..=14 | i @ 17 | i @ 29 | i @ 30 => {
                panic!("entry {} is an exception with error code", i)
            }
            i @ 18 => panic!("entry {} is an diverging exception (must not return)", i),
            i => panic!("no entry with index {}", i),
        }
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
pub struct IdtEntry<F> {
    offset_lower: u16,
    gdt_selector: u16,
    options: IdtEntryOptions,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
    phantom: PhantomData<F>,
}

impl<F> IdtEntry<F> {
    /// Creates a non-present IDT entry (but sets the must-be-one bits).
    #[inline]
    pub const fn missing() -> Self {
        IdtEntry {
            offset_lower: 0,
            gdt_selector: 0,
            options: IdtEntryOptions::minimal(),
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
            phantom: PhantomData,
        }
    }

    #[inline]
    fn set_handler_addr(&mut self, addr: VirtAddr) -> &mut IdtEntryOptions {
        use x86_64::instructions::segmentation;

        let addr = addr.as_u64();

        self.offset_lower = addr as u16;
        self.offset_middle = (addr >> 16) as u16;
        self.offset_high = (addr >> 32) as u32;

        self.gdt_selector = segmentation::CS::get_reg().0;

        self.options.set_present(true);
        &mut self.options
    }
}

impl<T> PartialEq for IdtEntry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.offset_lower == other.offset_lower
            && self.gdt_selector == other.gdt_selector
            && self.options == other.options
            && self.offset_middle == other.offset_middle
            && self.offset_high == other.offset_high
            && self.reserved == other.reserved
    }
}

pub type HandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame);
pub type HandlerFuncWithErrCode = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
pub type PageFaultHandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
pub type DivergingHandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame) -> !;
pub type DivergingHandlerFuncWithErrCode = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64) -> !;

/// A general handler function for an interrupt or an exception with the interrupt/exceptions's index and an optional error code.
pub type GeneralHandlerFunc = fn(InterruptStackFrame, index: u8, error_code: Option<u64>);

macro_rules! impl_set_handler_fn {
    ($h:ty) => {
        impl IdtEntry<$h> {
            /// Set the handler function for the IDT entry and sets the present bit.
            ///
            /// For the code selector field, this function uses the code segment selector currently
            /// active in the CPU.
            ///
            /// The function returns a mutable reference to the entry's options that allows
            /// further customization.
            #[inline]
            pub fn set_handler_fn(&mut self, handler: $h) -> &mut IdtEntryOptions {
                let handler = VirtAddr::new(handler as u64);
                unsafe { self.set_handler_addr(handler) }
            }
        }
    };
}

impl_set_handler_fn!(HandlerFunc);
impl_set_handler_fn!(HandlerFuncWithErrCode);
// impl_set_handler_fn!(PageFaultHandlerFunc);
impl_set_handler_fn!(DivergingHandlerFunc);
impl_set_handler_fn!(DivergingHandlerFuncWithErrCode);

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
#[repr(u8)]
pub enum PrivilegeLevel {
    /// Privilege-level 0 (most privilege): This level is used by critical system-software
    /// components that require direct access to, and control over, all processor and system
    /// resources. This can include BIOS, memory-management functions, and interrupt handlers.
    Ring0 = 0,

    /// Privilege-level 1 (moderate privilege): This level is used by less-critical system-
    /// software services that can access and control a limited scope of processor and system
    /// resources. Software running at these privilege levels might include some device drivers
    /// and library routines. The actual privileges of this level are defined by the
    /// operating system.
    Ring1 = 1,

    /// Privilege-level 2 (moderate privilege): Like level 1, this level is used by
    /// less-critical system-software services that can access and control a limited scope of
    /// processor and system resources. The actual privileges of this level are defined by the
    /// operating system.
    Ring2 = 2,

    /// Privilege-level 3 (least privilege): This level is used by application software.
    /// Software running at privilege-level 3 is normally prevented from directly accessing
    /// most processor and system resources. Instead, applications request access to the
    /// protected processor and system resources by calling more-privileged service routines
    /// to perform the accesses.
    Ring3 = 3,
}

impl PrivilegeLevel {
    /// Creates a `PrivilegeLevel` from a numeric value. The value must be in the range 0..4.
    ///
    /// This function panics if the passed value is >3.
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

/// Wrapper type for the interrupt stack frame pushed by the CPU.
///
/// This type derefs to an [`InterruptStackFrameValue`], which allows reading the actual values.
///
/// This wrapper type ensures that no accidental modification of the interrupt stack frame
/// occurs, which can cause undefined behavior (see the [`as_mut`](InterruptStackFrame::as_mut)
/// method for more information).
#[repr(C)]
pub struct InterruptStackFrame {
    value: InterruptStackFrameValue,
}


impl core::ops::Deref for InterruptStackFrame {
    type Target = InterruptStackFrameValue;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl core::fmt::Debug for InterruptStackFrame {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InterruptStackFrameValue {
    /// This value points to the instruction that should be executed when the interrupt
    /// handler returns. For most interrupts, this value points to the instruction immediately
    /// following the last executed instruction. However, for some exceptions (e.g., page faults),
    /// this value points to the faulting instruction, so that the instruction is restarted on
    /// return. See the documentation of the [`InterruptDescriptorTable`] fields for more details.
    pub instruction_pointer: VirtAddr,
    /// The code segment selector, padded with zeros.
    pub code_segment: u64,
    /// The flags register before the interrupt handler was invoked.
    pub cpu_flags: u64,
    /// The stack pointer at the time of the interrupt.
    pub stack_pointer: VirtAddr,
    /// The stack segment descriptor at the time of the interrupt (often zero in 64-bit mode).
    pub stack_segment: u64,
}

impl core::fmt::Debug for InterruptStackFrameValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        struct Hex(u64);
        impl core::fmt::Debug for Hex {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        let mut s = f.debug_struct("InterruptStackFrame");
        s.field("instruction_pointer", &self.instruction_pointer);
        s.field("code_segment", &self.code_segment);
        s.field("cpu_flags", &Hex(self.cpu_flags));
        s.field("stack_pointer", &self.stack_pointer);
        s.field("stack_segment", &self.stack_segment);
        s.finish()
    }
}