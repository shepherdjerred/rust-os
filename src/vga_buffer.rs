use volatile::Volatile;
use core::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Character {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<Character>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    current_column_position: usize,
    color_code: ColorCode,
    text_buffer: &'static mut Buffer,
}


impl Writer {
    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not printable ASCII
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.current_column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.current_column_position;

                let color_code = self.color_code;
                self.text_buffer.chars[row][col].write(Character {
                    ascii_character: byte,
                    color_code
                });
                self.current_column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.text_buffer.chars[row][col].read();
                self.text_buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT -1);
        self.current_column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blankCharacter = Character {
            ascii_character: b' ',
            color_code: self.color_code
        };
        for col in 0..BUFFER_WIDTH {
            self.text_buffer.chars[row][col].write(blankCharacter);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_string(string);
        Ok(())
    }
}

pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        current_column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        text_buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_string("This is a VGA text buffer!!");
    writer.new_line();
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}
