use std::io::{stdin, Write};
use termion::input::TermRead;
use termion::event::{Key, Event};
use crate::data::Data;
use termion::cursor;

pub fn input<T: Write>(data: &mut Data, screen: &mut T) {
    let stdin = stdin();

    for event in stdin.events() {
        match event.unwrap() {
            Event::Key(Key::Ctrl('q')) => { break; }
            Event::Key(Key::Down) => { data.move_down(); }
            Event::Key(Key::Up) => { data.move_up(); }
            Event::Key(Key::Right) => { data.move_right(); }
            Event::Key(Key::Left) => { data.move_left(); }
            Event::Key(Key::Char(_c)) => {}
            _ => {}
        }

        let _ =
            write!(
                screen,
                "{}{}",
                cursor::Goto(data.position.col as u16, data.position.row as u16),
                cursor::Show
            );
        screen.flush().unwrap();
    }
}