use crate::data::Data;
use std::io::{stdin, Write, Seek, SeekFrom};
use termion::event::{Event, Key};
use termion::input::TermRead;
use std::fs::File;

pub fn input<T: Write>(data: &mut Data, screen: &mut T, filename: &String) {
    let stdin = stdin();

    for event in stdin.events() {
        match event.unwrap() {
            Event::Key(Key::Ctrl('q')) => { return; }
            Event::Key(Key::Ctrl('w')) => { save(&data.bin, filename) }
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

fn save(buf: &Vec<u8>, filename: &String) {
    let mut file = File::create(filename).unwrap();
    let _ = file.seek(SeekFrom::Start(0));
    let result = file.write_all(buf);
    match result {
        Ok(_) => println!("save"),
        Err(e) => println!("{}", e)
    }
}