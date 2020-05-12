use std::io::{stdin, Write};
use termion::input::TermRead;
use termion::event::{Key, Event};
use crate::data::Data;

pub fn input(data: &mut Data, screen: &mut dyn Write) {
    let stdin = stdin();

    for event in stdin.events() {
        match event.unwrap() {
            Event::Key(Key::Ctrl('q')) => { break; }
            Event::Key(Key::Down) => {}
            Event::Key(Key::Up) => {}
            Event::Key(Key::Right) => {}
            Event::Key(Key::Left) => {}
            Event::Key(Key::Char(c)) => {}
            _ => {}
        }
    }
}