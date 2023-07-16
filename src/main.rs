use crate::components::{Memory, ProgramCounter};

mod components;
mod font;

fn main() {
    let memory = Memory::new();
    assert!(memory.length() == 4096);

    let font = font::Font::new();
    assert_eq!(
        font.get_character(&'1').unwrap().clone(),
        &[0x20, 0x60, 0x20, 0x20, 0x70]
    );

    let mut program_counter = ProgramCounter::new();
    assert_eq!(program_counter.get_count(), 0);

    program_counter.clock();
    program_counter.clock();
    assert_eq!(program_counter.get_count(), 2);
    println!("program counter count: {}", program_counter.get_count());

    program_counter.clear();
    assert_eq!(program_counter.get_count(), 0);
    println!("program counter count: {}", program_counter.get_count());
}
