use termion::screen::AlternateScreen;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

pub fn screen() {
    let mut screen = AlternateScreen::from(stdout());
    write!(screen, "alt now").unwrap();
    screen.flush().unwrap();

    thread::sleep(Duration::from_secs(3));
}