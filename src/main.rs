use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{self, Read},
    process,
};

use crate::chip8::chip8;
// use crate::clock::clock_emulator;
// use crate::display::Display;
// use crate::memory::Memory;
// use crate::program_counter::ProgramCounter;

mod chip8;
mod clock;
mod components;
mod draw;
mod font;
// mod memory;
// mod program_counter;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

fn read_rom(path: String) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    // println!("file: {:?}", buffer);

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

    // let rom_file_path = args.get(1).unwrap();
    println!("{}", rom_file_path);
    let rom = read_rom(rom_file_path).unwrap();

    // let memory = Memory::new();
    // assert!(memory.length() == 4096);

    // let mut program_counter = ProgramCounter::new();
    // assert_eq!(program_counter.get_count(), 0);

    // program_counter.clock();
    // program_counter.clock();
    // assert_eq!(program_counter.get_count(), 2);
    // println!("program counter count: {}", program_counter.get_count());

    // program_counter.clear();
    // assert_eq!(program_counter.get_count(), 0);
    // println!("program counter count: {}", program_counter.get_count());

    chip8(WIDTH, HEIGHT, rom);

    println!("exiting program");
}
