use std::collections::HashMap;

pub struct Font<'a> {
    char_set: HashMap<&'a str, [&'a str; 5]>,
}
impl<'a> Font<'a> {
    pub fn new() -> Font<'a> {
        let font: HashMap<&str, [&'a str; 5]> = HashMap::from([
            ("0", ["0xF0", "0x90", "0x90", "0x90", "0xF0"]),
            ("1", ["0x20", "0x60", "0x20", "0x20", "0x70"]),
            ("2", ["0xF0", "0x10", "0xF0", "0x80", "0xF0"]),
        ]);

        // self.char_set
        Font { char_set: font }
    }

    pub fn get_character(&self, symbol: &str) -> Result<&[&str; 5], String> {
        let character = self.char_set.get(symbol);

        match character {
            Some(byte_map) => Ok(byte_map),
            None => Err(String::from("No Character Found")),
        }
    }
}
