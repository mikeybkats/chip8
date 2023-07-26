use crate::chip8::chip8;
// use crate::chip8::chip8;
use crate::display::Display;
use crate::memory::Memory;
// use crate::program_counter::ProgramCounter;

mod chip8;
mod display;
mod font;
mod memory;
mod program_counter;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

fn main() {
    let memory = Memory::new();
    assert!(memory.length() == 4096);

    let font = font::Font::new();
    assert_eq!(
        font.get_character(&'1').unwrap().clone(),
        &[0x20, 0x60, 0x20, 0x20, 0x70]
    );

    let sprite = font.get_font_sprite(&'0').unwrap();
    println!("font sprite: {:?}", sprite);

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
}
