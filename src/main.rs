/*
    id like to apologize in advance for the terrible code, this is my first time writing Rust LOL
*/
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use patternscan::scan;
use std::io::{self, BufRead, Write, SeekFrom, Seek};

fn write_bytes(mut file: File, offset: u64, bytes: &[u8]) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(offset))?; 
    file.write_all(bytes)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let argv = args[1].clone();

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(argv)?;

    println!("patching...");

    let patched_byte : &[u8] = &[0x80, 0xbf, 0x78, 0x01, 0x00, 0x00, 0x00, 0x75, 0x05, 0xe8];
    let pattern = "80 bf 78 01 00 00 00 74 05 e8";
    let get_offset = scan(&file, &pattern).unwrap();

    println!("0x{:#X}", get_offset[0]);
    write_bytes(file, get_offset[0] as u64, patched_byte)?; // fuck

    println!("patched! you may close out of the program now");

    //exit program after input detected
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).expect("Failed to read line");

    Ok(())
}
