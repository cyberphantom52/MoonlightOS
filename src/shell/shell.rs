use lazy_static::lazy_static;

use crate::locks::mutex::Mutex;
use crate::println;
use crate::vga_buffer::{Color, WRITER};

const PROMPT: &str = "MoonlightOS> ";

lazy_static! {
    pub static ref SHELL: Mutex<Shell> = Mutex::new(Shell {
        buffer: [0 as char; 256],
        cursor: 0,
    });
}

pub struct Shell {
    buffer: [char; 256],
    cursor: usize,
}

impl Shell {
    pub fn init(&mut self) {
        self.buffer = [0 as char; 256];
        self.cursor = 0;

        let mut writer = WRITER.lock();
        writer.set_colors(Color::Pink, Color::Black);
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

    fn interpret(&mut self) {
        match self.buffer {
            _b if self.is_command("help") => {
                Shell::help();
            }
            _b if self.is_command("osinfo") => {
                Shell::osinfo();
            }
            _b if self.is_command("echo") => {
                Shell::echo(&self);
            }
            _b if self.is_command("clear") => {
                Shell::clear();
            }
            _ => println!("Unknown command!"),
        }
    }

    fn is_command(&self, command: &str) -> bool {
        for (i, c) in command.chars().enumerate() {
            if c != self.buffer[i] {
                return false;
            }
        }
        true
    }

    //commands
    fn help() {
        let mut writer = WRITER.lock();
        let border = "+-------------------------------------------+";

        writer.write_string(border);
        writer.new_line();
        writer.write_string("| Available commands:                       |");
        writer.new_line();
        writer.write_string("| ping  --> prints pong                     |");
        writer.new_line();
        writer.write_string("| echo  --> prints any string               |");
        writer.new_line();
        writer.write_string("| help  --> lists available commands        |");
        writer.new_line();
        writer.write_string("| clear --> clears the screen               |");
        writer.new_line();
        writer.write_string("| osinfo --> prints OS information          |");
        writer.new_line();
        writer.write_string(border);
        writer.new_line();
        drop(writer);
    }

    fn echo(&self) {
        let mut message_started = false;
        if self.buffer[self.cursor - 1] != '"' {
            let mut writer = WRITER.lock();
            writer.set_colors(Color::Pink, Color::Black);
            writer.write_string("Unknown command!");
            writer.reset_colors();
            writer.new_line();
            drop(writer);
            return;
        }
        for i in 0..self.cursor {
            let c = self.buffer[i];

            if c == '"' {
                if message_started {
                    let mut writer = WRITER.lock();
                    writer.new_line();
                    drop(writer);
                    break;
                } else {
                    message_started = true;
                }
            } else if message_started {
                let mut writer = WRITER.lock();
                writer.write_char(c);
                drop(writer);
            }
        }
    }

    fn clear() {
        let mut writer = WRITER.lock();
        writer.clear_screen();
        drop(writer);
    }

    fn osinfo() {
        const OSINFO_ASCII_ART: &str = r#"
        __  __                   _ _       _     _    ___  ____  
       |  \/  | ___   ___  _ __ | (_) __ _| |__ | |_ / _ \/ ___| 
       | |\/| |/ _ \ / _ \| '_ \| | |/ _` | '_ \| __| | | \___ \ 
       | |  | | (_) | (_) | | | | | | (_| | | | | |_| |_| |___) |
       |_|  |_|\___/ \___/|_| |_|_|_|\__, |_| |_|\__|\___/|____/ 
                                     |___/      
"#;

        let mut writer = WRITER.lock();
        writer.set_colors(Color::Cyan, Color::Black);
        writer.write_string(OSINFO_ASCII_ART);
        writer.reset_colors();
        writer.new_line();
        writer.write_string("OS Name: MoonlightOS");
        writer.new_line();
        writer.write_string("OS Version: 1.0.0");
        writer.new_line();
        drop(writer);
    }
}
