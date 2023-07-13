use crate::components::Memory;

mod components;
mod font;

fn main() {
    let memory = Memory::new();
    assert!(memory.length() == 4096);

    let font = font::Font::new();
    assert_eq!(
        font.get_character(&'1').unwrap().clone(),
        &[0x20, 0x60, 0x20, 0x20, 0x70]
    )
}
