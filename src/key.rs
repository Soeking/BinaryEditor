use std::io::stdin;
use termion::input::TermRead;
use termion::event::{Key, Event};

pub fn input() {
    let stdin = stdin();

    for event in stdin.events() {
        match event.unwrap() {
            Event::Key(Key::Ctrl('q')) => { break; }
            _ => { break; }
        }
    }
}