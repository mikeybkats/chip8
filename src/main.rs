use crate::display::Display;
use crate::memory::Memory;
use crate::program_counter::ProgramCounter;

mod display;
mod font;
mod memory;
mod program_counter;

fn main() {
    let memory = Memory::new();
    assert!(memory.length() == 4096);

    let font = font::Font::new();
    assert_eq!(
        font.get_character(&'1').unwrap().clone(),
        &[0x20, 0x60, 0x20, 0x20, 0x70]
    );

    font.draw_character(&'0');

    // let mut program_counter = ProgramCounter::new();
    // assert_eq!(program_counter.get_count(), 0);

    // program_counter.clock();
    // program_counter.clock();
    // assert_eq!(program_counter.get_count(), 2);
    // println!("program counter count: {}", program_counter.get_count());

    // program_counter.clear();
    // assert_eq!(program_counter.get_count(), 0);
    // println!("program counter count: {}", program_counter.get_count());

    // let display = Display::new(64, 32);
    // display.loop_window().unwrap();
}
