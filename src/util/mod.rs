pub(crate) trait U8 {
    fn to_hex(&self) -> char;
}

impl U8 for u8 {
    fn to_hex(&self) -> char {
        return match self {
            32 => { ' ' }
            33..=126 => { self.clone() as char }
            _ => { '.' }
        };
    }
}