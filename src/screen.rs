use super::*;
use crate::data::Data;
use std::fs::File;
use std::io::*;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

pub fn screen(file: &mut File, filename: &String) {
    let mut data = Data::new();
    let _ = file.read_to_end(&mut data.bin);
    data.init();

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    data.draw(&mut screen);

    key::input(&mut data, &mut screen, filename);
}
