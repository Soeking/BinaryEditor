use std::io::Write;
use termion::*;

pub struct Position {
    pub row: u32,
    pub col: u8,
    pub col_id: u8,
    pub char_pos: [u8; 48],
}

pub struct Data {
    pub bin: Vec<u8>,
    pub ascii: Vec<char>,
    pub position: Position,
    pub max_row: usize,
}

impl Data {
    pub fn new() -> Data {
        let bin: Vec<u8> = Vec::new();
        let ascii: Vec<char> = Vec::new();
        let pos: [u8; 48] = [11, 12, 14, 15, 17, 18, 20, 21, 23, 24, 26, 27, 29, 30, 32, 33,
            35, 36, 38, 39, 41, 42, 44, 45, 47, 48, 50, 51, 53, 54, 56, 57,
            62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77];
        Data { bin, ascii, position: Position { row: 1, col: 11, col_id: 0, char_pos: pos }, max_row: 1 }
    }

    pub fn init(&mut self) {
        let bytes = self.bin.clone();
        for x in bytes {
            match x {
                32 => { self.ascii.push(' ') }
                33..=126 => { self.ascii.push(x as char) }
                _ => { self.ascii.push('.') }
            }
        }
        self.max_row = self.bin.len() / 16 + 1;
    }

    pub fn draw<T: Write>(&mut self, screen: &mut T) {
        let mut address = 0;
        for i in 0..self.max_row {
            let _ = write!(screen, "{}", cursor::Goto(1, (i + 1) as u16)).unwrap();
            let _ = write!(screen, "{:>08x}  ", address);

            let range =
                if i != self.max_row - 1 { i * 16..(i + 1) * 16 } else { i * 16..self.bin.len() };

            for x in range.clone() {
                let _ = write!(screen, "{:>02x} ", &self.bin[x]);
            }

            let _ = write!(screen, "{}|", cursor::Goto(61, (i + 1) as u16));
            for x in range.clone() {
                let _ = write!(screen, "{}", &self.ascii[x]);
            }
            let _ = write!(screen, "|");
            address += 16;
        }
        let _ = write!(screen, "{}{}{}", cursor::Goto(11, 1), cursor::BlinkingBlock, cursor::Show);
    }

    pub fn move_down(&mut self) {
        if self.position.row < self.max_row as u32 {
            self.position.row += 1;
        }
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
    }

    pub fn move_left(&mut self) {
        if self.position.col_id > 0 {
            self.position.col_id -= 1;
            self.position.col = self.position.char_pos[self.position.col_id as usize];
        }
    }

    pub fn backspace(&mut self) {}

    pub fn delete(&mut self) {}

    pub fn insert(&mut self, c: char) {}
}