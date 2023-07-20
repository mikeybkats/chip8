// Memory: CHIP-8 has direct access to up to 4 kilobytes of RAM
pub struct Memory {
    mem: [i8; 4096],
}
impl Memory {
    // the first 512 blocks of memory are empty, because the original chip8 used these to store the interpreter software
    pub fn new() -> Memory {
        Memory { mem: [0; 4096] }
    }

    pub fn length(&self) -> usize {
        self.mem.len()
    }
}
