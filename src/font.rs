use std::collections::HashMap;

use crate::draw::Drawable;

#[derive(Debug)]
pub struct FontSprite {
    pub character: u8,
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl Drawable for FontSprite {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

pub const CHAR_SET: [u8; 16] = [
    0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
];

pub fn get_character_set() -> HashMap<u8, [u8; 5]> {
    HashMap::from([
        (0x0, [0xF0, 0x90, 0x90, 0x90, 0xF0]),
        (0x1, [0x20, 0x60, 0x20, 0x20, 0x70]),
        (0x2, [0xF0, 0x10, 0xF0, 0x80, 0xF0]),
        (0x3, [0xF0, 0x10, 0xF0, 0x10, 0xF0]),
        (0x4, [0x90, 0x90, 0xF0, 0x10, 0x10]),
        (0x5, [0xF0, 0x80, 0xF0, 0x10, 0xF0]),
        (0x6, [0xF0, 0x80, 0xF0, 0x90, 0xF0]),
        (0x7, [0xF0, 0x10, 0x20, 0x40, 0x40]),
        (0x8, [0xF0, 0x90, 0xF0, 0x90, 0xF0]),
        (0x9, [0xF0, 0x90, 0xF0, 0x10, 0xF0]),
        (0xA, [0xF0, 0x90, 0xF0, 0x90, 0x90]),
        (0xB, [0xE0, 0x90, 0xE0, 0x90, 0xE0]),
        (0xC, [0xF0, 0x80, 0x80, 0x80, 0xF0]),
        (0xD, [0xE0, 0x90, 0x90, 0x90, 0xE0]),
        (0xE, [0xF0, 0x80, 0xF0, 0x80, 0xF0]),
        (0xF, [0xF0, 0x80, 0xF0, 0x80, 0x80]),
    ])
}

/**
 * Font the typeface program for chip 8
 */
pub struct Font {
    char_set: HashMap<u8, [u8; 5]>,
    sprites: HashMap<u8, FontSprite>,
}
impl Font {
    pub fn new() -> Font {
        let char_set = get_character_set();

        let mut sprites: HashMap<u8, FontSprite> = HashMap::new();

        for (key, val) in char_set.iter() {
            let sprite = FontSprite {
                character: *key,
                width: 8,
                height: 5,
                pixels: Self::convert_font_to_sprite(&*val),
            };

            sprites.insert(*key, sprite);
        }

        Font { char_set, sprites }
    }

    /**
     * get_character
     * get a raw [u8; 5] character from the character set.
     */
    pub fn get_character(&self, symbol: &u8) -> Result<&[u8; 5], String> {
        let character = self.char_set.get(symbol);

        match character {
            Some(byte_map) => Ok(byte_map),
            None => Err(String::from("No Character Found")),
        }
    }

    /**
     * convert_font_to_sprite
     * converts a raw [u8; 5] font character to an image sprite vector array
     * the binary in the vector array represent the pixel map for the font
     * fonts are 8 bits wide by 5 bits tall, but the last four bits of width are all zeros and only present to conform to the 8 bit architecture of chip 8.
     */
    pub fn convert_font_to_sprite(font_symbol: &[u8]) -> Vec<u8> {
        let mut pixels: Vec<u8> = Vec::new();
        for row in font_symbol.iter() {
            // {:b} is binary format
            // {:08b} formats the number as 8-bit binary with leading zeros
            let binary_string = format!("{:08b}", row);

            for bit in binary_string.chars() {
                if let Some(bit_value) = bit.to_digit(2) {
                    pixels.push(bit_value as u8);
                }
            }
        }
        pixels
    }

    /**
     * get_font_sprite
     * returns the font sprite struct for the given character
     * The data should be stored in the interpreter area of Chip-8 memory (0x000 to 0x1FF)
     */
    pub fn get_font_sprite(&self, symbol: &u8) -> Option<&FontSprite> {
        self.sprites.get(symbol)
    }
}
