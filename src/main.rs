use crate::components::Memory;

mod components;

fn main() {
    let memory = Memory::new();
    assert!(memory.length() == 4096);
}
