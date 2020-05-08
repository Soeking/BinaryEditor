use termion::screen::AlternateScreen;
use std::io::{stdout, Write, Read};
use super::key;
use std::fs::File;

pub fn screen(file: &mut File) {
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);

    let mut screen = AlternateScreen::from(stdout());
    write!(screen, "{:?}", buf).unwrap();
    screen.flush().unwrap();

    key::input();
}