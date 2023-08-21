use pixels::Pixels;

use crate::{
    draw::{Draw, Point},
    font::Font,
    program_counter::ProgramCounter,
    registers::Registers,
    stack::Stack,
};

// TODO: how does execute get access to all the methods it needs?
pub fn execute(
    instruction: u16,
    stack: &mut Stack,
    registers: &mut Registers,
    program_counter: &mut ProgramCounter,
    pixels: &mut Pixels,
    width: u32,
    height: u32,
) {
    /*
     * NNN: address
     * NN: 8-bit constant
     * N: 4-bit constant
     * X and Y: 4-bit register identifier
     * PC : Program Counter
     * I : 16bit register (For memory address) (Similar to void pointer);
     * VN: One of the 16 available variables. N may be 0 to F (hexadecimal);
     */
    let first_nibble = (instruction >> 12) & 0xF;

    let screen = pixels.frame_mut();

    let mut draw = Draw::new(width, height, screen);

    match first_nibble {
        // 0 Calls machine code routine at address NNN - not be needed for emulator
        0x0 => {
            // 00E0 - clears screen
            match instruction {
                0x00E0 => draw.clear(),
                0x00EE => {
                    // Return from a subroutine.
                    // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
                    let current_address = stack.current();
                    program_counter.jump(current_address);
                }
                _ => (),
            }
        }

        // 1NNN Jumps to address at NNN
        // The interpreter sets the program counter to nnn.
        0x1 => {
            // 178D
            let location = instruction & 0xFFF;
            program_counter.jump(location as usize);
        }

        // 2NNN Calls subroutine at NNN
        // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
        0x2 => {
            // increment stack pointer
            let new_current = program_counter.increment();
            // push current PC to top of stack
            stack.push(new_current);
            // set PC to NNN
            let nnn = instruction & 0xFFF;
            program_counter.jump(nnn as usize)
        }

        // 3xkk - SE Vx, byte
        // Skip next instruction if Vx = kk.
        // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2
        0x3 => {
            // use bit shift and mask to get the desired 4 bits
            let vx_register = instruction >> 8 & 0xF;
            // get vx
            let vx = *registers.get_register(vx_register).unwrap();
            // get kk
            let kk = (instruction & 0xFF) as u8;
            // compares
            if kk == vx {
                // skip next instruction by incrementing PC by two
                program_counter.increment();
                program_counter.increment();
            }
        }

        // 4XNN Skips the next instruction if VX does not equal NN (usually the next instruction is a jump to skip a code block).
        0x4 => {
            // use bit shift and mask to get the desired 4 bits
            let vx_register = instruction >> 8 & 0xF;
            // get vx
            let vx = *registers.get_register(vx_register).unwrap();
            // get kk
            let kk = (instruction & 0xFF) as u8;
            // compares
            if kk != vx {
                // skip next instruction by incrementing PC by two
                program_counter.increment();
                program_counter.increment();
            }
        }

        // 5XY0 Skips the next instruction if VX equals VY (usually the next instruction is a jump to skip a code block).
        // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
        0x5 => {
            let x_register = instruction >> 8 & 0xF;
            let vx = *registers.get_register(x_register).unwrap();
            let vy_register = instruction >> 4 & 0xF;
            let vy = *registers.get_register(vy_register).unwrap();

            if vx == vy {
                program_counter.increment();
                program_counter.increment();
            }
        }

        // 6XNN Sets VX to NN.
        0x6 => {
            let nn = ((instruction as usize) & 0xFF) as u8;
            let vx_register = instruction >> 8 & 0xF;
            registers.set_register(vx_register, nn);
        }

        // 7XNN Adds NN to VX (carry flag is not changed).
        0x7 => {
            let nn = ((instruction as usize) & 0xFF) as u8;
            let vx_register = instruction >> 8 & 0xF;
            let vx_value = registers.get_register(vx_register).unwrap();
            registers.set_register(vx_register, nn + vx_value);
        }

        0x8 => {
            let instruction_0 = instruction & 0xF;
            let vy_index = instruction >> 4 & 0xF;
            let vy_value = *registers.get_register(vy_index).unwrap();
            let vx_index = instruction >> 8 & 0xF;
            let vx_value = *registers.get_register(vx_index).unwrap();
            match instruction_0 {
                0 => {
                    // 8XY0 Sets VX to the value of VY.
                    registers.set_register(vx_index, vy_value);
                }
                1 => {
                    // 8XY1 Sets VX to VX or VY. (bitwise OR operation)
                    registers.set_register(vx_index, vx_value | vy_value);
                }

                2 => {
                    // 8XY2 Sets VX to VX and VY. (bitwise AND operation)
                    registers.set_register(vx_index, vx_value & vy_value);
                }

                3 => {
                    // 8XY3 Sets VX to VX xor VY.
                    registers.set_register(vx_index, vx_value ^ vy_value);
                }

                4 => {
                    // 8XY4 Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
                    let (sum, overflow) = vy_value.overflowing_add(vx_value);
                    registers.set_register(vx_index, sum);

                    if overflow {
                        registers.set_register(0xF, 1);
                    }
                }

                5 => {
                    // 8XY5 VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                    let (diff, borrow) = vx_value.overflowing_sub(vy_value);
                    registers.set_register(vx_index, diff);

                    if borrow {
                        registers.set_register(0xF, 0);
                    }
                }

                6 => {
                    // 8XY6 Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
                    let vx_lsb = vx_value & 1;
                    registers.set_register(0xF, vx_lsb);
                    registers.set_register(vx_index, vx_value >> 1);
                }

                7 => {
                    // 8XY7 Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not
                }

                8 => {
                    // 8XYE	Stores the most significant bit of VX in VF and then shifts VX to the left by 1
                }
                _ => (),
            }
        }

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

    // TODO: shouldn't have to call this here. Why do i have to?
    // calling pixels.render() forces the render
    pixels.render().unwrap();
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
