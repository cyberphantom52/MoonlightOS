#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    Pink = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCode(u8);

// Color bit of vga buffer uses MSB to determine if the char is blinking
// The next three bits are used to set the background color
// And the remaining 4 bits are used to set the foreground color
impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
// Each character in the vga buffer is of 2 bytes with the first being the char and 2nd being the byte
pub struct ScreenChar {
    ascii_char: u8,
    pub color_code: ColorCode
}

// Standard size of VGA Buffer
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// https://os.phil-opp.com/vga-text-mode/#volatile
// Compiler doesn’t know that we really access VGA buffer memory (instead of normal RAM) and knows nothing about the side effect that some characters appear on the screen. 
// So it might decide that these writes are unnecessary and can be omitted. 
// Using volatile crate to avoid erroneous optimization of the Buffer
use volatile::Volatile;
#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}


pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer
}


// https://os.phil-opp.com/vga-text-mode/#lazy-statics
// Statics are initialized at compile time.
// The problem here is that Rust is not able to convert raw pointers to references at compile time.
// lazy_static! macro , instead of computing its value at compile time, initializes itself when accessed for the first time.

// https://os.phil-opp.com/vga-text-mode/#spinlocks
// Writer is immutable by default which is pretty useless.
// A `static mut` can solve the problem its highly discouraged as it can lead to data races.
// An alternative is to use a spinlock. 
use lazy_static::lazy_static;
use super::locks::mutex::Mutex;
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        // 0xb8000 MMIO address for vga buffer
        // https://os.phil-opp.com/vga-text-mode/#the-vga-text-buffer
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });    
}

impl Writer {

    // pub fn clear_screen(&mut self) {
    //     for row in 0..BUFFER_HEIGHT {
    //         self.clear_row(row);
    //     }
    //     self.column_position = 0;
    // }

    pub fn set_colors(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }
    pub fn reset_colors(&mut self) {
        self.color_code = ColorCode::new(Color::Yellow, Color::Black);
    }
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\x08' => self.backspace(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                // Always print at the last line of buffer
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code,
                });
                
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn backspace(&mut self) {
        if self.column_position > 0 {
            self.column_position -= 1;
            let row = BUFFER_HEIGHT - 1;
            let col = self.column_position;
            let blank = ScreenChar {
                ascii_char: b'\0',
                color_code: self.color_code,
            };
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}



// Implemet fmt::Writer for our Writer so we can use rust's formatting macros like write!/writeln!
use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Marco overrides for standard print macros so we can use them again.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;   

    interrupts::without_interrupts(|| {     
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_char), c);
        }
    });
}
