use std::env;
use std::fs::File;

mod screen;
mod key;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let filename = args.get(1).unwrap();
        let file = File::open(filename);
        screen::screen(&mut file.unwrap());
    } else {
        println!("no file");
    }
}
