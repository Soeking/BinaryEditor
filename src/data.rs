pub  struct Position {
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
        let mut bin: Vec<u8> = Vec::new();
        let mut ascii: Vec<char> = Vec::new();
        Data { bin, ascii, position: Position { row: 1, col: 1 } }
    }

    pub fn insert() {}
}