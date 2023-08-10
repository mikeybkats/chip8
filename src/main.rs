use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    process,
};

use crate::chip8::chip8;

mod chip8;
mod components;
mod draw;
mod font;
mod memory;
mod program_counter;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

fn read_rom(path: String) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    // let file_path = "roms/output.txt";
    // let mut file_out = File::create(file_path)?;
    // let hex_string: String = buffer.iter().map(|&byte| format!("{:02X}", byte)).collect();
    // file_out.write_all(hex_string.as_bytes())?;
    // file_out.flush()?;

    Ok(buffer)
}

fn check_args(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("Not enough arguments");
    }
    Ok(args[1].clone())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let rom_file_path = check_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("{}", rom_file_path);
    let rom = read_rom(rom_file_path).unwrap();

    chip8(WIDTH, HEIGHT, rom);

    println!("exiting program");
}
