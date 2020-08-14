use crate::data::Data;
use std::io::{stdin, Write, Seek, SeekFrom};
use termion::event::{Event, Key};
use termion::input::TermRead;
use std::fs::File;
use termion::*;

pub fn input<T: Write>(data: &mut Data, screen: &mut T, filename: &String) {
    let stdin = stdin();

    for event in stdin.events() {
        match event.unwrap() {
            Event::Key(Key::Ctrl('q')) => { return; }
            Event::Key(Key::Ctrl('w')) => { save(&data.bin, screen, filename) }
            Event::Key(Key::Down) => {
                data.move_down();
                data.show_cursor(screen);
            }
            Event::Key(Key::Up) => {
                data.move_up();
                data.show_cursor(screen);
            }
            Event::Key(Key::Right) => {
                data.move_right();
                data.show_cursor(screen);
            }
            Event::Key(Key::Left) => {
                data.move_left();
                data.show_cursor(screen);
            }
            Event::Key(Key::Backspace) => {
                data.backspace();
                data.draw(screen);
            }
            Event::Key(Key::Delete) => {
                data.delete();
                data.draw(screen);
            }
            Event::Key(Key::Char(_c)) => {
                data.insert(_c);
                data.draw(screen);
            }
            _ => {}
        }
    }
}

fn save<T: Write>(buf: &Vec<u8>, screen: &mut T, filename: &String) {
    let size = terminal_size().unwrap().1;
    let mut file = File::create(filename).unwrap();
    let _ = file.seek(SeekFrom::Start(0));

    let result = file.write_all(buf);
    match result {
        Ok(_) => { let _ = write!(screen, "{}:saved", cursor::Goto(1, size)); }
        Err(e) => println!("{}", e)
    }
    screen.flush().unwrap();
}