use crate::print;
use crate::vga_buffer::Color;
use crate::vga_buffer::WRITER;
use core::fmt::Write;

pub fn run_shell() {
    // loop {
    let mut writer = WRITER.lock();

    writer.set_colors(Color::Pink, Color::Black);
    write!(writer, "MoonlightOS> ").unwrap();
    writer.set_colors(Color::White, Color::Black);

    // Release the lock to allow keyboard input
    drop(writer);
    let mut command_buffer = [0; 16];
    let command_length = read_input(&mut command_buffer);
    process_command(&command_buffer[..command_length]);

    //clear command buffer
    // for i in 0..command_length {
    //     command_buffer[i] = 0;
    // }
    // command_length = 0;
    // println!("{}",i);
    // i += 1;
    // }
}

fn read_input(buffer: &mut [u8; 16]) -> usize {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use x86_64::instructions::port::Port;

    let mut buffer_index = 0;
    let mut finished = false;
    let mut last_scancode = None; // Add this variable

    while !finished && buffer_index < buffer.len() {
        let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);
        let mut port = Port::new(0x60);

        let scancode: u8 = unsafe { port.read() };

        // Check if this scancode is same as the previous one (key repeat)
        if last_scancode == Some(scancode) {
            continue;
        }
        last_scancode = Some(scancode);

        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        if character == '\n' {
                            finished = true;
                        } else if character == '\u{8}' {
                            if buffer_index > 0 {
                                buffer_index -= 1;
                                let mut writer = WRITER.lock();
                                writer.write_byte(b'\x08');
                            }
                        } else {
                            buffer[buffer_index] = character as u8;
                            print!("{}", character);
                            buffer_index += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    buffer_index
}

fn process_command(command: &[u8]) {
    let mut writer = WRITER.lock();

    // Convert the command buffer to a string
    let command_str = core::str::from_utf8(command).unwrap();

    match command_str {
        "ping" => {
            writer.set_colors(Color::White, Color::Black);
            writeln!(writer, "\npong").unwrap();
            writer.reset_colors();
        }
        "hello" => {
            writer.set_colors(Color::White, Color::Black);
            writeln!(writer, "\nHello, world!").unwrap();
            writer.reset_colors();
        }
        "help" => {
            writer.set_colors(Color::White, Color::Black);
            writeln!(writer, "\nCommands:").unwrap();
            writeln!(writer, "ping - pong").unwrap();
            writeln!(writer, "hello - hello, world!").unwrap();
            writeln!(writer, "help - this help message").unwrap();
            writeln!(writer, "matrix - matrix rain").unwrap();
            writer.reset_colors();
        }
        "matrix" => {
            writer.set_colors(Color::LightGreen, Color::Black);
            let mut j = 0;
            loop {
                match j {
                    0 => writeln!(writer, "      $  ^   $      $   $").unwrap(),
                    1 => writeln!(writer, "       &    o       &   *      O   ").unwrap(),
                    2 => writeln!(writer, "  Y    !   &   &     $    P     L").unwrap(),
                    3 => writeln!(writer, "  ^   $   &   !   *       (    ").unwrap(),
                    _ => j = 0,
                }
                j += 1;
                // Add a delay here if desired
            }
            writer.reset_colors();
        },
        // "clear" => {
        //     let mut writer = WRITER.lock();
        
        //     // Clear the screen
        //     writer.clear_screen();
        
        //     // Print a new prompt
        //     writer.set_colors(Color::Pink, Color::Black);
        //     writeln!(writer, "MoonlightOS> ").unwrap();
        //     writer.set_colors(Color::White, Color::Black);
        // },
        _ => {
            writer.set_colors(Color::Magenta, Color::Black);
            writeln!(writer, "\nUnknown command").unwrap();
            writer.reset_colors();
        }
    }
}
