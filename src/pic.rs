//                      ____________                          ____________
// Real Time Clock --> |            |   Timer -------------> |            |
// ACPI -------------> |            |   Keyboard-----------> |            |      _____
// Available --------> | Secondary  |----------------------> | Primary    |     |     |
// Available --------> | Interrupt  |   Serial Port 2 -----> | Interrupt  |---> | CPU |
// Mouse ------------> | Controller |   Serial Port 1 -----> | Controller |     |_____|
// Co-Processor -----> |            |   Parallel Port 2/3 -> |            |
// Primary ATA ------> |            |   Floppy disk -------> |            |
// Secondary ATA ----> |____________|   Parallel Port 1----> |____________|

// Two PIC chips --> PIC1 and PIC2
//PIC2 is slaved to interrupt2 of PIC1

//Default offset of PIC1 is 0x20-0x27
//Default offset of PIC2 is 0x28-0x2F

// Chip - Purpose	I/O port
// Master PIC - Command	0x0020
// Master PIC - Data	0x0021
// Slave PIC - Command	0x00A0
// Slave PIC - Data	0x00A1

use core::arch::asm;

/// Constants for PIC initialization and control.
const PIC_INIT: u8 = 0x11;
const PIC_EOI: u8 = 0x20;
const MODE_8086: u8 = 0x01;

/// PIC port addresses.
const MASTER_PIC_CMD_PORT: u8 = 0x20;
const MASTER_PIC_DATA_PORT: u8 = 0x21;
const SLAVE_PIC_CMD_PORT: u8 = 0xA0;
const SLAVE_PIC_DATA_PORT: u8 = 0xA1;

struct Pic {
    offset: u8,
    command: u8,
    data: u8,
}

impl Pic {
    /// Returns whether the PIC handles the given interrupt.
    fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        (self.offset <= interrupt_id) && (interrupt_id < self.offset + 8)
    }

    unsafe fn end_of_interrupt(&mut self) {
        unsafe {
            asm!("out dx, al", in("dx") self.command as u16, in("al") PIC_EOI);
        }
    }

    /// Read the mask of the PIC.
    unsafe fn read_mask(&mut self) -> u8 {
        let mask: u8;
        unsafe {
            asm!("in al, dx", out("al") mask, in("dx") self.data as u16);
        }

        mask
    }

    /// Write the given mask to the PIC.
    unsafe fn write_mask(&mut self, mask: u8) {
        unsafe {
            asm!("out dx, al", in("dx") self.data as u16, in("al") mask);
        }
    }

    /// Send the given command to the PIC.
    unsafe fn send(&mut self, command: u8) {
        unsafe {
            asm!("out dx, al", in("dx") self.command as u16, in("al") command);
        }
    }
}

pub struct ChainedPics {
    slave: Pic,
    master: Pic,
}

impl ChainedPics {
    /// Create a new PIC structure.
    pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            master: Pic {
                offset: offset1,
                command: MASTER_PIC_CMD_PORT,
                data: MASTER_PIC_DATA_PORT,
            },
            slave: Pic {
                offset: offset2,
                command: SLAVE_PIC_CMD_PORT,
                data: SLAVE_PIC_DATA_PORT,
            },
        }
    }

    /// Initialize the PICs.
    pub unsafe fn initialize(&mut self) {
        let mask1 = self.master.read_mask();
        let mask2 = self.slave.read_mask();

        self.master.send(PIC_INIT);
        wait();
        self.slave.send(PIC_INIT);
        wait();

        self.master.write_mask(self.master.offset);
        wait();
        self.slave.write_mask(self.slave.offset);
        wait();

        self.master.write_mask(4);
        wait();
        self.slave.write_mask(2);
        wait();

        self.master.write_mask(MODE_8086);
        wait();
        self.slave.write_mask(MODE_8086);
        wait();

        self.master.write_mask(mask1);
        self.slave.write_mask(mask2);
    }

    /// Returns whether the PIC handles the given interrupt.
    fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.master.handles_interrupt(interrupt_id) || self.slave.handles_interrupt(interrupt_id)
    }

    /// Notify the PIC that an interrupt has been handled.
    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.handles_interrupt(interrupt_id) {
            if self.master.handles_interrupt(interrupt_id) {
                self.master.end_of_interrupt();
            }
            self.slave.end_of_interrupt();
        }
    }
}

// Wait for I/O operation to complete by writing to an unused port.
fn wait() {
    unsafe {
        asm!("out dx, al", in("dx") 0x80 as u16, in("al") 0x00 as u8);
    }
}
