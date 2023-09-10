use lazy_static::lazy_static;

use crate::locks::mutex::Mutex;
use crate::vga_buffer::{Color, WRITER};

const PROMPT: &str = "MoonlightOS> ";

lazy_static! {
    pub static ref SHELL: Mutex<Shell> = Mutex::new(Shell {
        buffer: [0 as char; 256],
        arg: [0 as char; 11],
        cursor: 0,
    });
}

pub struct Shell {
    buffer: [char; 256],
    arg: [char; 11],
    cursor: usize,
}

impl Shell {
    pub fn init(&mut self) {
        self.buffer = [0 as char; 256];
        self.cursor = 0;

        let mut writer = WRITER.lock();
        writer.set_colors(Color::LightGreen, Color::Black);
        writer.write_string(PROMPT);
        writer.reset_colors();
        drop(writer);
    }

    pub fn add(&mut self, c: char) {
        self.buffer[self.cursor] = c;
        self.cursor += 1;

        let mut writer = WRITER.lock();
        writer.write_char(c);
        drop(writer);
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.buffer[self.cursor] = 0 as char;
            self.cursor -= 1;

            let mut writer = WRITER.lock();
            writer.backspace();
            drop(writer);
        }
    }

    //shell enter
    pub fn enter(&mut self) {
        let mut writer = WRITER.lock();
        writer.new_line();
        drop(writer);

        self.interpret();
        self.init();
    }

    pub fn interpret(&mut self) {
        let mut writer = WRITER.lock();
        writer.write_string("Unknown command!");
        writer.new_line();
        drop(writer);
    }
}
