use lazy_static::lazy_static;

use crate::locks::mutex::Mutex;
use crate::vga_buffer::{Color, WRITER};
use crate::{print, println};

const PROMPT: &str = "MoonlightOS> ";
const HELP: &'static str = "+-------------------------------------------+
| Available commands:                       |
| echo  --> prints any string               |
| help  --> lists available commands        |
| clear --> clears the screen               |
| osinfo --> prints OS information          |
+-------------------------------------------+
";

lazy_static! {
    pub static ref SHELL: Mutex<Shell> = Mutex::new(Shell {
        buffer: ['\0'; 256],
        cursor: 0,
    });
}

pub struct Shell {
    buffer: [char; 256],
    cursor: usize,
}

impl Shell {
    pub fn init(&mut self) {
        self.buffer = ['\0'; 256];
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

        WRITER.lock().write_char(c);
    }

    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        
        self.buffer[self.cursor] = '\0';
        self.cursor -= 1;
        WRITER.lock().backspace();
    }

    //shell enter
    pub fn enter(&mut self) {
        WRITER.lock().new_line();

        self.interpret();
        self.init();
    }

    fn interpret(&mut self) {
        match self.buffer {
            _b if self.is_command("help") => print!("{}", HELP),
            _b if self.is_command("osinfo") => self.osinfo(),
            _b if self.is_command("echo") => self.echo(),
            _b if self.is_command("clear") => self.clear(),
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
    fn echo(&self) {
        let mut writer = WRITER.lock();
        for c in self.buffer.iter().skip(5) {
            if *c == '\0' {
                break;
            }
            writer.write_char(*c);
        }
        writer.new_line();
        drop(writer);
    }

    fn clear(&self) {
        WRITER.lock().clear_screen();
    }

    fn osinfo(&self) {
        const OSINFO_ASCII_ART: &str = r#"
        __  __                   _ _       _     _    ___  ____  
       |  \/  | ___   ___  _ __ | (_) __ _| |__ | |_ / _ \/ ___| 
       | |\/| |/ _ \ / _ \| '_ \| | |/ _` | '_ \| __| | | \___ \ 
       | |  | | (_) | (_) | | | | | | (_| | | | | |_| |_| |___) |
       |_|  |_|\___/ \___/|_| |_|_|_|\__, |_| |_|\__|\___/|____/ 
                                     |___/      
"#;

        let mut writer = WRITER.lock();
        writer.set_colors(Color::Red, Color::Black);
        writer.write_string(OSINFO_ASCII_ART);
        writer.reset_colors();
        writer.write_string("OS Name: MoonlightOS\n");
        writer.write_string("OS Version: 0.1.0\n");
        drop(writer);
    }
}
