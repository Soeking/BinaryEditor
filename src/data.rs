use std::io::Write;
use termion::*;
use super::util::Util;

pub struct Position {
    pub row: u32,
    pub col: u8,
    pub col_id: u8,
    pub char_pos: [u8; 48],
}

pub struct Data {
    pub bin: Vec<u8>,
    pub ascii: Vec<char>,
    pub chars: Vec<char>,
    pub position: Position,
    pub max_row: usize,
}

impl Data {
    pub fn new() -> Data {
        let bin = Vec::new();
        let ascii = Vec::new();
        let chars = Vec::new();
        let pos: [u8; 48] = [11, 12, 14, 15, 17, 18, 20, 21, 23, 24, 26, 27, 29, 30, 32, 33,
            36, 37, 39, 40, 42, 43, 45, 46, 48, 49, 51, 52, 54, 55, 57, 58,
            62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77];
        Data { bin, ascii, chars, position: Position { row: 1, col: 11, col_id: 0, char_pos: pos }, max_row: 1 }
    }

    pub fn init(&mut self) {
        let bytes = self.bin.clone();
        for x in bytes {
            self.ascii.push(x.to_hex());

            format!("{:>02x}", x).chars().for_each(|c| self.chars.push(c));
        }
    }

    pub fn draw<T: Write>(&mut self, screen: &mut T) {
        self.max_row = (self.bin.len() + 15) / 16;
        let mut address: u64 = 0;

        for i in 0..self.max_row {
            let _ = write!(screen, "{}", cursor::Goto(1, (i + 1) as u16)).unwrap();
            let _ = write!(screen, "{:>08x}  ", address);

            let range =
                if i != self.max_row - 1 { i * 32..(i + 1) * 32 } else { i * 32..self.chars.len() };

            for x in range.clone() {
                let _ = write!(screen, "{}", &self.chars[x]);
                if x & 1 == 1 { let _ = write!(screen, " "); }
                if x % 16 == 15 { let _ = write!(screen, " "); }
            }

            let _ = write!(screen, "{}|", cursor::Goto(61, (i + 1) as u16));
            let range =
                if i != self.max_row - 1 { i * 16..(i + 1) * 16 } else { i * 16..self.ascii.len() };
            for x in range.clone() {
                let _ = write!(screen, "{}", &self.ascii[x]);
            }
            let _ = write!(screen, "|");
            address += 16;
        }
        let _ = write!(
            screen,
            "{}{}{}",
            cursor::Goto(self.position.col as u16, self.position.row as u16),
            cursor::BlinkingBlock,
            cursor::Show);
        screen.flush().unwrap();
    }

    pub fn move_down(&mut self) {
        if self.position.row < self.max_row as u32 {
            self.position.row += 1;
        }
        if self.position.row == self.max_row as u32 { self.at_bottom('d') }
    }

    pub fn move_up(&mut self) {
        if self.position.row > 0 {
            self.position.row -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.position.col_id < 47 {
            self.position.col_id += 1;
            self.position.col = self.position.char_pos[self.position.col_id as usize];
        }
        if self.position.row == self.max_row as u32 { self.at_bottom('r') }
    }

    pub fn move_left(&mut self) {
        if self.position.col_id > 0 {
            self.position.col_id -= 1;
            self.position.col = self.position.char_pos[self.position.col_id as usize];
        }
        if self.position.row == self.max_row as u32 { self.at_bottom('l') }
    }

    fn at_bottom(&mut self, direction: char) {
        let length = self.chars.len() % 32;
        let before = &self.position.char_pos[0..length];
        let after = &self.position.char_pos[32..32 + (length / 2)];
        let positions = [before, after].concat() as Vec<u8>;
        if !positions.contains(&self.position.col) {
            if positions.is_empty() {
                self.position.row -= 1;
            } else {
                if self.position.col_id < 32 {
                    match direction {
                        'r' => { self.position.col_id = 32; }
                        'l' => { self.position.col_id = before.len() as u8 - 1 }
                        'd' => {
                            let pad = (32 - before.len()) / 2;
                            if self.position.col_id as usize >= before.len() + pad {
                                self.position.col_id = 32;
                            } else {
                                self.position.col_id = before.len() as u8 - 1;
                            }
                        }
                        _ => {}
                    }
                } else {
                    self.position.col_id = after.len() as u8 + 31;
                }
                self.position.col = self.position.char_pos[self.position.col_id as usize];
            }
        }
    }

    pub fn backspace(&mut self) {}

    pub fn delete(&mut self) {}

    pub fn insert(&mut self, _c: char) {
        if self.position.col_id < 32 {
            if ('0'..='9').contains(&_c) || ('a'..='f').contains(&_c) {
                let id = (self.position.row as usize - 1) * 32 + self.position.col_id as usize;
                self.chars.insert(id, _c);
                self.position.col_id += 1;
                if self.position.col_id == 32 {
                    self.position.col_id = 0;
                    self.position.row += 1;
                }
                self.position.col = self.position.char_pos[self.position.col_id as usize];
                self.change_bin();
            }
        } else {
            let id = (self.position.row as usize - 1) * 16 + self.position.col_id as usize - 32;
            self.ascii.insert(id, _c);
            self.bin.insert(id, _c as u8);

            let mut i = 0;
            let num = format!("{:>02x}", _c as u8);
            num.chars().for_each(|c| {
                self.chars.insert(id * 2 + i, c);
                i += 1;
            });

            self.position.col_id += 1;
            if self.position.col_id == 48 {
                self.position.col_id = 32;
                self.position.row += 1;
            }
            self.position.col = self.position.char_pos[self.position.col_id as usize];
        }
    }

    fn change_bin(&mut self) {
        let mut bin_id = if self.position.col_id <= 2 && self.position.row == 1 {
            0
        } else {
            (self.position.row as usize - 1) * 32 + self.position.col_id as usize - 2
        };
        let mut ascii_id = if self.position.col_id <= 2 && self.position.row == 1 {
            0
        } else {
            (self.position.row as usize - 1) * 16 + self.position.col_id as usize / 2 - 1
        };
        if bin_id & 1 == 1 {
            bin_id += 1;
            ascii_id += 1;
        }

        for i in (bin_id..self.chars.len()).step_by(2) {
            let f = if self.chars[i] >= 'a' {
                self.chars[i] as u8 - 97 + 10
            } else {
                self.chars[i] as u8 - 48
            };
            let num = if i == self.chars.len() - 1 {
                f * 16
            } else if self.chars[i + 1] >= 'a' {
                self.chars[i + 1] as u8 - 97 + 10 + f * 16
            } else {
                self.chars[i + 1] as u8 - 48 + f * 16
            };

            if ascii_id == self.bin.len() {
                self.bin.push(num);
                self.ascii.push(num.to_hex());
            } else {
                unsafe {
                    let id = self.bin.get_unchecked_mut(ascii_id);
                    *id = num;
                }

                unsafe {
                    let id = self.ascii.get_unchecked_mut(ascii_id);
                    *id = num.to_hex();
                }
            }
            ascii_id += 1;
        }
    }
}