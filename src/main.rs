use crate::chip8::chip8;
// use crate::chip8::chip8;
// use crate::chip8::chip8;
// use crate::clock::clock_emulator;
// use crate::display::Display;
// use crate::memory::Memory;
// use crate::program_counter::ProgramCounter;

mod chip8;
mod clock;
mod components;
mod draw;
mod font;
mod memory;
mod program_counter;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

fn main() {
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

    chip8(WIDTH, HEIGHT);

    println!("exiting program");
}
