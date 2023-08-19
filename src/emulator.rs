use crate::{
    draw::{Draw, Point},
    font::Font,
    program_counter::ProgramCounter,
    registers::{self, Registers},
    stack::Stack,
};

// TODO: how does execute get access to all the methods it needs?
pub fn execute(instruction: u16, _stack: &Stack, registers: &Registers) -> bool {
    // println!("Decoding");
    /*
     * NNN: address
     * NN: 8-bit constant
     * N: 4-bit constant
     * X and Y: 4-bit register identifier
     * PC : Program Counter
     * I : 16bit register (For memory address) (Similar to void pointer);
     * VN: One of the 16 available variables. N may be 0 to F (hexadecimal);
     *
     * X: The second nibble. Used to look up one of the 16 registers (VX) from V0 through VF.
     * Y: The third nibble. Also used to look up one of the 16 registers (VY) from V0 through VF.
     * N: The fourth nibble. A 4-bit number.
     * NN: The second byte (third and fourth nibbles). An 8-bit immediate number.
     * NNN: The second, third and fourth nibbles. A 12-bit immediate memory address.
     */
    // let case = instruction.chars().nth(0).unwrap();

    let first_nibble = (instruction >> 12) & 0xF;

    match first_nibble {
        // 0 Calls machine code routine at address NNN - not be needed for emulator
        0x0 => (),

        // 1NNN Jumps to address at NNN
        0x1 => {
            // 178D
            let _nnn = &instruction[1..3];
        }

        // 2NNN Calls subroutine at NNN
        0x2 => {
            let _nnn = &instruction[1..3];
        }

        // 3xkk - SE Vx, byte
        // Skip next instruction if Vx = kk.
        // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2
        0x3 => {
            // 3C42
            println!("instruction chars: {:?}", instruction.chars());
            let register = instruction.chars().nth(1).unwrap();
            println!("register: {}", register);

            let vx = registers.get_register(register).unwrap();
            println!("vx: {}", vx);

            // let instruction_character_2 = instruction.chars().nth(2).unwrap();
            // let instruction_character_3 = instruction.chars().nth(3).unwrap();
            // let kk = format!("{}{}", instruction_character_2, instruction_character_3);
            let kk = &instruction[2..4];
            println!("kk: {}", kk);

            if kk == vx {}
        }

        // 4XNN Skips the next instruction if VX does not equal NN (usually the next instruction is a jump to skip a code block).
        0x4 => (),

        // 5XY0 Skips the next instruction if VX equals VY (usually the next instruction is a jump to skip a code block).
        0x5 => (),

        // 6XNN Sets VX to NN.
        0x6 => (),

        // 7XNN Adds NN to VX (carry flag is not changed).
        0x7 => (),

        0x8 => (
            // 8XY0 Sets VX to the value of VY.
            // 8XY1 Sets VX to VX or VY. (bitwise OR operation)
            // 8XY2 Sets VX to VX and VY. (bitwise AND operation)
            // 8XY3 Sets VX to VX xor VY.
            // 8XY4 Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
            // 8XY5 VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
            // 8XY6 Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
            // 8XY7 Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not
            // 8XYE	Stores the most significant bit of VX in VF and then shifts VX to the left by 1
        ),

        // 9XY0 Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump to skip a code block);
        0x9 => (),

        // ANNN Sets I to the address NNN.
        0xA => (),

        // BNNN Jumps to the address NNN plus V0.
        0xB => (),

        // CXNN Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
        0xC => (),

        // DXYN Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value does not change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen.
        0xD => (),

        0xE => (
            // EX9E Skips the next instruction if the key stored in VX is pressed (usually the next instruction is a jump to skip a code block).
            // EXA1 Skips the next instruction if the key stored in VX is not pressed (usually the next instruction is a jump to skip a code block).
        ),

        0xF => (
            // FX07	Sets VX to the value of the delay timer.
            // FX0A	A key press is awaited, and then stored in VX (blocking operation, all instruction halted until next key event).
            // FX15	Sets the delay timer to VX.
            // FX18	Sets the sound timer to VX.
            // FX1E	Adds VX to I. VF is not affected.[c]
            // FX29	Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
            // FX33	Stores the binary-coded decimal representation of VX, with the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
            // FX55	Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
            // FX65	Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified.[d]
        ),
        _ => (),
    }
    false
}

pub fn _decode(_command: bool) -> bool {
    false
}

/* Fetches the program instruction from the chip8 Rom */
pub fn fetch(
    rom: &Vec<u8>,
    program_counter: &mut ProgramCounter,
    rom_length: usize,
) -> Option<u16> {
    if program_counter.get_pc() < rom_length - 1 {
        let instruction1 = *rom.get(program_counter.get_pc()).unwrap() as u16;
        program_counter.increment();
        let instruction2 = *rom.get(program_counter.get_pc()).unwrap() as u16;
        program_counter.increment();

        let instruction: u16 = (instruction1 << 8) | instruction2;
        Some(instruction)
    } else {
        None
    }
}

/* Fetches and formats the program instruction from the chip8 Rom */
pub fn fetch_instruction(
    rom: &Vec<u8>,
    program_counter: &mut ProgramCounter,
    rom_length: usize,
) -> u16 {
    match fetch(&rom, program_counter, rom_length) {
        // :04X specifies a width of 4 with leading zeros
        // Some(integer) => format!("{:04X}", integer),
        Some(integer) => integer,
        // _ => String::from("0000"),
        _ => 0x0,
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
