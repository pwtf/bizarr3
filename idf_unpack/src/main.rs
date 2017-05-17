extern crate byteorder;

use byteorder::{ReadBytesExt, LittleEndian};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::Cursor;
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
    let mut magic = [0u8; 4];

    let mut file = match File::open(&path) {
        Err(e) => panic!("Cannot open {}: {}", path.display(), e.description()),
        Ok(file) => file,
    };

    match file.read_exact(&mut magic) {
        Err(e) => panic!("Cannot read from {}: {}", path.display(), e.description()),
        Ok(_) => {
            if b"FFFL" == &magic {
                // Skipping the magic* 0x0000000c
                let _ = file.seek(SeekFrom::Current(4));
                let (offset, key) = header(&mut file);
                let toc = unpack_toc(&mut file, offset, key);

                // println!("t:{:08x} o:{:08x} k:{:08x}", idf_type, offset, key.unwrap());
            } else {
                panic!("Unknown file format {}: {:?}", path.display(), magic);
            }
        },
    };
}

fn header(file: &mut File) -> (u32, Option<u32>) {
    let idf_type = file.read_u32::<LittleEndian>().unwrap();
    let mut offset = file.read_u32::<LittleEndian>().unwrap();
    let mut key: Option<u32> = Some(file.read_u32::<LittleEndian>().unwrap());

    match idf_type {
        0x10000 => key = None,
        0x10100 => offset = offset ^ 0x123,
        _ => panic!("Unknown idf_type: {:?}", idf_type)
    }

    if key.is_some() && key.unwrap() >> 24 != 1 {
        key = None;
    }

    return (offset, key);
}

struct TocFile {
    name: String,
    offset: usize,
    size: usize
}
fn unpack_toc(file: &mut File, offset: u32, key: Option<u32>) -> &'static str {
    let _ = file.seek(SeekFrom::Start(offset as u64));
    let num_entries = file.read_u32::<LittleEndian>().unwrap() as usize;
    let mut toc_buffer = Vec::new();
    let mut decrypted_toc_buffer = Vec::new();
    let mut toc: Vec<TocFile> = Vec::new();

    {
        let reference = Read::by_ref(file);
        let _ = reference.take((num_entries * 0x40) as u64).read_to_end(&mut toc_buffer);
    }

    if key.is_some() {
        let mut k = 0x27 + (key.unwrap() & 0xff) as u8;

        for (i, byte) in toc_buffer.iter().enumerate() {
            let b = byte ^ k;
            decrypted_toc_buffer.push(b);
            k = b.wrapping_add(i as u8).wrapping_add(k.wrapping_mul(5));
        }
    }

    println!("{:?}", decrypted_toc_buffer);

    for i in 0..num_entries {
        let j = i * 0x40;
        let raw_name = decrypted_toc_buffer.get(j..j+0x38).unwrap();
        let mut raw_usizes = Cursor::new(decrypted_toc_buffer.get(j+0x38..j+0x40).unwrap());

        let offset = raw_usizes.read_u32::<LittleEndian>();
        let size = raw_usizes.read_u32::<LittleEndian>();
        let name = String::from_utf8({
            let mut n = Vec::new();
            n.extend_from_slice(raw_name);
            n
        });

        toc.push(TocFile {
            name: name.unwrap(),
            offset: offset.unwrap() as usize,
            size: size.unwrap() as usize
        });
    }

    println!(":{}:", toc[0].name);
    return "";
}
