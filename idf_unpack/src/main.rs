use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

fn main() {
    for arg in env::args().skip(1) {
        println!("{}", arg);
        process_file(arg);
    }
}

fn process_file(filename: String) {
    let path = Path::new(&filename);
    let mut magic = [0u8; 8];
    let mut file = match File::open(&path) {
        Err(e) => panic!("Cannot open {}: {}", path.display(), e.description()),
        Ok(file) => file,
    };

    match file.read_exact(&mut magic) {
        Err(e) => panic!("Cannot read from {}: {}", path.display(), e.description()),
        Ok(_) => {
            print!("Magic bytes:");
            for x in &magic {
                print!(" {:02x}", x);
            };
            println!();
        },
    };
}
