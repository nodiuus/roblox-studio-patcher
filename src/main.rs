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

    println!("[*] patching...");

    let patched_bytes : &[u8] = &[0x75, 0x05, 0xE8, 0xB2, 0x98, 0xB7, 0x03, 0x48];
    let pattern = "74 05 E8 B2 98 B7 03 48";
    let get_offset = scan(&file, &pattern).unwrap();

    println!("{:#X}", get_offset[0]);
    write_bytes(file, get_offset[0] as u64, patched_byte)?; 

    println!("[+] patched! you may close out of the program now");

    //exit program after input detected
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).expect("Failed to read line");

    Ok(())
}
