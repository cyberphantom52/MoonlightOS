use core::arch::asm;

/// Halts the CPU until the next interrupt arrives.
#[inline]
pub fn hlt() {
    unsafe {
        asm!("hlt", options(nomem, nostack, preserves_flags));
    }
}

/// Executes the `nop` instructions, which performs no operation (i.e. does nothing).
#[inline]
pub fn nop() {
    unsafe {
        asm!("nop", options(nomem, nostack, preserves_flags));
    }
}

/// Enable interrupts.
#[inline]
pub fn enable_interrupts() {
    unsafe {
        asm!("sti", options(nomem, nostack));
    }
}

/// Check if interrupts are enabled.
#[inline]
pub fn interrupts_enabled() -> bool {
    let r: u64;

    // Push rflags register onto the stack, then pop it into r.
    unsafe {
        asm!("pushfq; pop {}", out(reg) r, options(nomem, preserves_flags));
    }

    // Bit 9 is the interrupt flag.
    // https://wiki.osdev.org/CPU_Registers_x86#EFLAGS_Register
    (r >> 9) & 1 == 1
}

/// Disable interrupts.
#[inline]
pub fn disable_interrupts() {
    unsafe {
        asm!("cli", options(nomem, nostack));
    }
}

/// Cause a breakpoint exception by invoking the `int3` instruction.
#[inline]
pub fn int3() {
    unsafe {
        asm!("int3", options(nomem, nostack));
    }
}
