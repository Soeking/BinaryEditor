use termion::color;
use termion::screen::AlternateScreen;
use std::io::{stdout, Write};
use std::{thread, time};

fn main() {
    {
        let mut screen = AlternateScreen::from(stdout());
        write!(screen, "alt now").unwrap();
        screen.flush().unwrap();

        thread::sleep(time::Duration::from_secs(3));
    }
}
