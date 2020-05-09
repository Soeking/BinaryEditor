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
        Data { bin, ascii, position: Position { row: 1, col: 1 } }
    }

    pub fn init(&mut self) {
        let bytes = self.bin.clone();
        for x in bytes {
            match x {
                32 => { self.ascii.push(' ') }
                33...126 => { self.ascii.push(x as char) }
                _ => { self.ascii.push('.') }
            }
        }
    }

    pub fn draw(&mut self) {
        let max_row = &self.bin.len() / 16 + 1;
    }

    pub fn insert() {}
}