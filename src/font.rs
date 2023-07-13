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
}
