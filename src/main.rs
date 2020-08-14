use std::env;
use std::fs::*;

mod data;
mod key;
mod screen;
pub mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let filename = args.get(1).unwrap();
        let file = OpenOptions::new().read(true).write(true).create(true).open(filename);
        match file {
            Ok(mut f) => screen::screen(&mut f, filename),
            Err(e) => println!("{}", e)
        }
    } else {
        println!("input file");
    }
}
