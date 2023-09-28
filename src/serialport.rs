use ::core::arch::asm;
// const COM_PORT: u8 = 0x3f8;

pub struct SerialPort {
    /// The data to be transmitted or received over the serial port.
    data: u16,
    interrupt_enable: u16,
    baud_rate_divisor_ls: u16,
    baud_rate_divisor_ms: u16,
    fifo_control: u16,
    line_control: u16,
    modem_control: u16,
    line_status: u16,
    modem_statis: u16,
    scratch: u16,
}

impl SerialPort {
    pub const unsafe fn new(com_port: u16) -> Self {
        Self {
            data: com_port,
            interrupt_enable: com_port + 1,
            baud_rate_divisor_ls: com_port + 0,
            baud_rate_divisor_ms: com_port + 1,
            fifo_control: com_port + 2,
            line_control: com_port + 3,
            modem_control: com_port + 4,
            line_status: com_port + 5,
            modem_statis: com_port + 6,
            scratch: com_port + 7,
        }
    }

    pub fn init_serial(&mut self) {
        unsafe {
            asm!("out dx, al", in("dx") self.interrupt_enable as u16, in("al") 0x00 as u8);
            asm!("out dx, al", in("dx") self.line_control as u16, in("al") 0x80 as u8);
            asm!("out dx, al", in("dx") self.baud_rate_divisor_ls as u16, in("al") 0x03 as u8);
            asm!("out dx, al", in("dx") self.baud_rate_divisor_ms as u16, in("al") 0x00 as u8);
            asm!("out dx, al", in("dx") self.line_control as u16, in("al") 0x03 as u8);
            asm!("out dx, al", in("dx") self.fifo_control as u16, in("al") 0xC7 as u8);
            asm!("out dx, al", in("dx") self.modem_control as u16, in("al") 0x0B as u8);
            asm!("out dx, al", in("dx") self.modem_control as u16, in("al") 0x1E as u8);
            asm!("out dx, al", in("dx") self.interrupt_enable as u16, in("al") 0xAE as u8);
        }
    }

    //Recieving Data

    /// This function checks if there is any data received on the serial port and returns the number of bytes received.
    /// * `i32` - number of bytes received on the serial port
    fn serial_recieved(&mut self) -> i32 {
        let value: u8;
        unsafe {
            asm!(
                "in al, dx",
                in("dx") self.line_status as u16,
                out("al") value,
            );
        }
        (value & 1) as i32
    }

    /// Reads a character from the serial port.
    fn read_serial(&mut self) -> char {
        loop {
            if SerialPort::serial_recieved(self) != 0 {
                break;
            }
        }
        let value: u8;
        unsafe {
            asm!(
                "in al, dx",
                in("dx") self.data as u16,
                out("al") value,
            );
        }
        value as char
    }

    //Sending Data

    // Checks if the transmit buffer is empty.
    fn is_transmit_empty(&mut self) -> i32 {
        let value: u8;
        unsafe {
            asm!(
                "in al, dx",
                in("dx") self.line_status as u16,
                out("al") value,
            );
        }
        (value & 0x20) as i32
    }

    // Writes a character to the serial port.
    fn write_serial(&mut self, a: char) {
        while self.is_transmit_empty() == 0 {}
        unsafe {
            asm!(
                "out dx,al",
                in("dx") self.data as u16,
                in("al") a as u8
            );
        }
    }
}

use core::fmt;
impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_serial(byte as char);
        }
        Ok(())
    }
}
