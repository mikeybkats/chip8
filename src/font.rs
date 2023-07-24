use std::collections::HashMap;

pub struct Font<'a> {
    char_set: HashMap<char, &'a [u8; 5]>,
}
impl<'a> Font<'a> {
    pub fn new() -> Font<'a> {
        let font: HashMap<char, &[u8; 5]> = HashMap::from([
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

        Font { char_set: font }
    }

    pub fn get_character(&self, symbol: &char) -> Result<&&[u8; 5], String> {
        let character = self.char_set.get(symbol);

        match character {
            Some(byte_map) => Ok(byte_map),
            None => Err(String::from("No Character Found")),
        }
    }

    pub fn get_font_sprite(&self, symbol: &char) {
        let character_arr = self.char_set.get(symbol);

        if let Some(character) = character_arr.as_ref() {
            for row in character.iter() {
                // {:b} is binary format
                let binary_string = format!("{:b}", row);
                let mut binary_digits = Vec::new();

                let mut count = 0;
                for digit in binary_string.chars() {
                    if count < 4 {
                        binary_digits.push(digit);
                        // TODO: draw the pixel here
                    } else {
                        break;
                    }
                    count += 1;
                }

                println!("{:?}", binary_digits)
            }
        }
    }
}
