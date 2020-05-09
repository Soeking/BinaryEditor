use std::io::Write;
use termion::*;

pub struct Position {
    row: u32,
    col: u8,
}

pub struct Data {
    pub bin: Vec<u8>,
    pub ascii: Vec<char>,
    pub position: Position,
}

impl Data {
    pub fn new() -> Data {
        let bin: Vec<u8> = Vec::new();
        let ascii: Vec<char> = Vec::new();
        Data { bin, ascii, position: Position { row: 1, col: 11 } }
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
    }

    pub fn draw<T: Write>(&mut self, screen: &mut T) {
        let max_row = &self.bin.len() / 16 + 1;
        let mut address = 0;
        for i in 0..max_row {
            write!(screen, "{}", cursor::Goto(1, (i + 1) as u16)).unwrap();
            write!(screen, "{:>08x}  ", address);

            let range =
                if i != max_row - 1 { i * 16..(i + 1) * 16 } else { i * 16..self.bin.len() };

            for x in range.clone() {
                write!(screen, "{:>02x} ", &self.bin[x]);
            }

            if i == max_row - 1 {
                for x in 0..(i + 1) * 16 - self.bin.len() {
                    write!(screen, "   ");
                }
            }

            write!(screen, "  |");
            for x in range.clone() {
                write!(screen, "{} ", &self.ascii[x]);
            }
            write!(screen, "|");
            address += 16;
        }
    }

    pub fn insert() {}
}