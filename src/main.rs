use std::{
    env,
    fs::File,
    io::{self, Read},
    process,
};

use crate::chip8::chip8;

mod chip8;
mod display;
mod draw;
mod emulator;
mod font;
mod memory;
mod program_counter;
mod registers;
mod stack;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

fn read_rom(path: String) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn main() {
    // Initialize the logger
    // env_logger::builder()
    //     .filter_level(log::LevelFilter::Info) // Set the log level
    //     .init();

    let args: Vec<String> = env::args().collect();

    let rom = args
        .get(1)
        .map(|path| read_rom(path.clone()).unwrap_or_else(|e| {
            eprintln!("Could not read ROM at {path}: {e}");
            process::exit(1);
        }));

    chip8(WIDTH, HEIGHT, rom);

    println!("exiting program");
}
