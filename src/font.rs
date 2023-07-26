use std::collections::HashMap;

use crate::display::Drawable;

#[derive(Debug)]
pub struct FontSprite {
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

pub struct Font<'a> {
    char_set: HashMap<char, &'a [u8; 5]>,
    sprites: HashMap<char, FontSprite>,
}
impl<'a> Font<'a> {
    pub fn new() -> Font<'a> {
        let char_set: HashMap<char, &[u8; 5]> = HashMap::from([
            ('0', &[0xF0, 0x90, 0x90, 0x90, 0xF0]),
            ('1', &[0x20, 0x60, 0x20, 0x20, 0x70]),
            ('2', &[0xF0, 0x10, 0xF0, 0x80, 0xF0]),
            ('3', &[0xF0, 0x10, 0xF0, 0x10, 0xF0]),
            ('4', &[0x90, 0x90, 0xF0, 0x10, 0x10]),
            ('5', &[0xF0, 0x80, 0xF0, 0x10, 0xF0]),
            ('6', &[0xF0, 0x80, 0xF0, 0x90, 0xF0]),
            ('7', &[0xF0, 0x10, 0x20, 0x40, 0x40]),
            ('8', &[0xF0, 0x90, 0xF0, 0x90, 0xF0]),
            ('9', &[0xF0, 0x90, 0xF0, 0x10, 0xF0]),
            ('A', &[0xF0, 0x90, 0xF0, 0x90, 0x90]),
            ('B', &[0xE0, 0x90, 0xE0, 0x90, 0xE0]),
            ('C', &[0xF0, 0x80, 0x80, 0x80, 0xF0]),
            ('D', &[0xE0, 0x90, 0x90, 0x90, 0xE0]),
            ('E', &[0xF0, 0x80, 0xF0, 0x80, 0xF0]),
            ('F', &[0xF0, 0x80, 0xF0, 0x80, 0x80]),
        ]);

        let mut sprites: HashMap<char, FontSprite> = HashMap::new();

        for (key, val) in char_set.iter() {
            let sprite = FontSprite {
                width: 8,
                height: 5,
                pixels: Self::convert_font_to_sprite(*val),
            };

            sprites.insert(key.clone(), sprite);
        }

        Font { char_set, sprites }
    }

    pub fn get_character(&self, symbol: &char) -> Result<&&[u8; 5], String> {
        let character = self.char_set.get(symbol);

        match character {
            Some(byte_map) => Ok(byte_map),
            None => Err(String::from("No Character Found")),
        }
    }

    fn convert_font_to_sprite(font_symbol: &[u8]) -> Vec<u8> {
        let mut pixels: Vec<u8> = Vec::new();
        for row in font_symbol.iter() {
            // {:b} is binary format
            let binary_string = format!("{:b}", row);

            for bit in binary_string.chars() {
                if let Some(bit_value) = bit.to_digit(2) {
                    pixels.push(bit_value as u8);
                }
            }
        }
        // println!("{:?}", pixels);
        pixels
    }

    pub fn get_font_sprite(&self, symbol: &char) -> Option<&FontSprite> {
        // let character_arr = self.char_set.get(symbol);

        self.sprites.get(symbol)
    }
}
