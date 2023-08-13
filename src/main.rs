use std::{
    env,
    fs::File,
    io::{self, Read},
    process,
};

use crate::chip8::chip8;

mod chip8;
mod components;
mod draw;
mod font;
mod memory;
mod program_counter;
mod utils;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

fn read_rom(path: String) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

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

    let rom = read_rom(rom_file_path).unwrap();

    chip8(WIDTH, HEIGHT, rom);

    println!("exiting program");
}
