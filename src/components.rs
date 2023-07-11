// Memory: CHIP-8 has direct access to up to 4 kilobytes of RAM
pub struct Memory {
    length: u16,
}
impl Memory {
    // the first 512 blocks of memory are empty, because the original chip8 used these to store the interpreter software
    pub fn new() -> Memory {
        Memory { length: (4096) }
    }

    pub fn length(&self) -> u16 {
        self.length
    }
}

// Display: 64 x 32 pixels (or 128 x 64 for SUPER-CHIP) monochrome, ie. black or white
pub struct Display {}
impl Display {}

// A program counter, often called just “PC”, which points at the current instruction in memory
pub struct ProgramCounter {}
impl ProgramCounter {}

// One 16-bit index register called “I” which is used to point at locations in memory
pub struct I {}
impl I {}

// A stack for 16-bit addresses, which is used to call subroutines/functions and return from them
pub struct Stack {}
impl Stack {}

// An 8-bit delay timer which is decremented at a rate of 60 Hz (60 times per second) until it reaches 0
pub struct DelayTimer {}
impl DelayTimer {}

// An 8-bit sound timer which functions like the delay timer, but which also gives off a beeping sound as long as it’s not 0
pub struct SoundTimer {}
impl SoundTimer {}

// 16 8-bit (one byte) general-purpose variable registers numbered 0 through F hexadecimal, ie. 0 through 15 in decimal, called V0 through VF
// VF is also used as a flag register; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag
pub struct GeneralRegisters {}
impl GeneralRegisters {}
