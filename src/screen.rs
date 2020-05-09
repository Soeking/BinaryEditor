use std::io::*;
use super::*;
use std::fs::File;
use termion::screen::AlternateScreen;
use crate::data::Data;
use termion::raw::IntoRawMode;

pub fn screen(file: &mut File) {
    let mut data = Data::new();
    let _ = file.read_to_end(&mut data.bin);
    data.init();

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    //write!(screen, "{}", termion::cursor::Hide).unwrap();
    data.draw(&mut screen);

    screen.flush().unwrap();

    key::input();
}