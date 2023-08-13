use crate::{
    draw::{Draw, Point},
    font::Font,
    program_counter::ProgramCounter,
};

pub fn decode(instruction: String) -> bool {
    // println!("Decoding");
    let case = instruction.chars().nth(0).unwrap();

    match case {
        '0' => (),
        '1' => (),
        '2' => (),
        '3' => (),
        '4' => (),
        '5' => (),
        '6' => (),
        '7' => (),
        '8' => (),
        '9' => (),
        'A' => (),
        'B' => (),
        'C' => (),
        'D' => (),
        'E' => (),
        'F' => (),
        _ => (),
    }
    false
}

pub fn execute(_command: bool) -> bool {
    // println!("Executing");
    false
}

/* Fetches the program instruction from the chip8 Rom */
pub fn fetch(rom: &[u8], program_counter: &mut ProgramCounter, rom_length: usize) -> Option<u16> {
    if program_counter.get_index() < rom_length - 1 {
        let instruction1 = rom[program_counter.get_index()];
        program_counter.increment();
        let instruction2 = rom[program_counter.get_index()];
        program_counter.increment();

        let instruction: u16 = ((instruction1 as u16) << 8) | instruction2 as u16;
        Some(instruction)
    } else {
        None
    }
}

/* Fetches and formats the program instruction from the chip8 Rom */
pub fn fetch_instruction(
    rom: &[u8],
    program_counter: &mut ProgramCounter,
    rom_length: usize,
) -> String {
    match fetch(&rom, program_counter, rom_length) {
        Some(integer) => format!("{:X}", integer),
        _ => String::from("0000"),
    }
}

/** Prints a font ramp and pixels to demonstrate the edge of the four screen corners */
pub fn test_print(width: u32, height: u32, screen: &mut [u8]) {
    let mut draw = Draw::new(width, height, screen);

    draw.draw_pixel(&Point { x: 0, y: 0 });
    draw.draw_pixel(&Point { x: 63, y: 0 });
    draw.draw_pixel(&Point { x: 0, y: 31 });
    draw.draw_pixel(&Point { x: 63, y: 31 });

    let font = Font::new();
    assert_eq!(
        font.get_character(&'1').unwrap().clone(),
        &[0x20, 0x60, 0x20, 0x20, 0x70]
    );

    let char_set = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    let mut count = 0;
    let mut y = 3;
    for character in char_set {
        let mut x = 2 + (count * 5);
        if count == 10 {
            count = 0;
            x = 2 + count * 5;
            y = 10;
        }
        draw.blit_drawable(&Point { x, y }, font.get_font_sprite(&character).unwrap());
        count += 1;
    }
}
