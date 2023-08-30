// // Memory: CHIP-8 has direct access to up to 4 kilobytes of RAM
// /**
// +---------------+= 0xFFF (4095) End of Chip-8 RAM
// |               |
// |               |
// |               |
// |               |
// |               |
// | 0x200 to 0xFFF|
// |     Chip-8    |
// | Program / Data|
// |     Space     |
// |               |
// |               |
// |               |
// +- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
// |               |
// |               |
// |               |
// +---------------+= 0x200 (512) Start of most Chip-8 programs
// | 0x000 to 0x1FF|
// | Reserved for  |
// |  interpreter  |
// +---------------+= 0x000 (0) Start of Chip-8 RAM
// */
// // static mut RAM: [u8; 4096] = [0; 4096];

use crate::font::{get_character_set, CHAR_SET};

pub struct Memory {
    ram: [u8; 4096],
}
impl Memory {
    // the first 512 blocks of memory are empty, because the original chip8 used these to store the interpreter software
    pub fn new() -> Memory {
        let ram: [u8; 4096] = [0; 4096];
        Memory { ram }
    }

    pub fn set_fonts(&mut self) {
        let fonts = get_character_set();
        let ram_len = self.ram.len();

        for (index, char) in CHAR_SET.iter().enumerate() {
            let base_index = index * 5;

            let font = fonts.get(char).unwrap();

            if base_index + 5 <= ram_len {
                self.ram[base_index..base_index + 5].copy_from_slice(font);
            }
        }
    }

    /** Returns the RAM value at the given address. */
    fn _peek(&self, address: usize) -> u8 {
        self.ram[address]
    }

    /** Sets the RAM value at the given address to the given value. */
    fn _poke(&mut self, address: usize, value: u8) {
        self.ram[address] = value;
    }

    pub fn get_memory(&mut self) -> &mut [u8] {
        &mut self.ram
    }

    /** Sets a rom to the program space in the chip8 memory */
    pub fn set_rom(&mut self, rom: &Vec<u8>) -> Result<&[u8], String> {
        let rom_len = rom.len();
        let end_index = 512 + rom_len;
        if rom_len <= self.ram.len() - 512 {
            self.ram[512..end_index].copy_from_slice(&rom);
            Ok(&self.ram)
        } else {
            Err(String::from(
                "Not enough space in the array to copy new values.",
            ))
        }
    }
}
