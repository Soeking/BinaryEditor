use termion::screen::AlternateScreen;
use std::io::{stdout, Write};
use super::key;

pub fn screen() {
    let mut screen = AlternateScreen::from(stdout());
    write!(screen, "alt now").unwrap();
    screen.flush().unwrap();

    key::input();
}