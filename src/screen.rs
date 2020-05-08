use std::io::*;
use super::*;
use std::fs::File;
use termion::screen::AlternateScreen;
use termion::cursor;
use crate::data::Data;

pub fn screen(file: &mut File) {
    let mut data = Data::new();
    let _ = file.read_to_end(&mut data.bin);

    let mut screen = AlternateScreen::from(stdout());
    write!(screen, "{}", cursor::Goto(1, 1)).unwrap();
    write!(screen, "{:?}", data.bin).unwrap();
    screen.flush().unwrap();

    key::input();
}