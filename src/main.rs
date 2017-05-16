use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    let path = Path::new("Text.idf");
    let mut magic = [0; 8];
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
