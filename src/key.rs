use crate::data::Data;
use std::io::{stdin, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;

pub fn input<T: Write>(data: &mut Data, screen: &mut T) {
    let stdin = stdin();

    for event in stdin.events() {
        match event.unwrap() {
            Event::Key(Key::Ctrl('q')) => { return; }
            Event::Key(Key::Ctrl('w')) => {}
            Event::Key(Key::Down) => { data.move_down(); }
            Event::Key(Key::Up) => { data.move_up(); }
            Event::Key(Key::Right) => { data.move_right(); }
            Event::Key(Key::Left) => { data.move_left(); }
            Event::Key(Key::Backspace) => { data.backspace(); }
            Event::Key(Key::Delete) => { data.delete(); }
            Event::Key(Key::Char(_c)) => { data.insert(_c); }
            _ => {}
        }
        data.draw(screen);
    }
}
