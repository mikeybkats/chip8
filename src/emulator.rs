use pixels::Pixels;
use winit::event::ScanCode;

use crate::{
    draw::{Draw, Point, Sprite},
    font::{Font, CHAR_SET},
    memory::Memory,
    program_counter::ProgramCounter,
    registers::Registers,
    stack::Stack,
};

pub fn execute(
    instruction: u16,
    memory: &mut Memory,
    stack: &mut Stack,
    registers: &mut Registers,
    program_counter: &mut ProgramCounter,
    pixels: &mut Pixels,
    width: u32,
    height: u32,
    key_state: KeyPress,
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

    let vy_index = (instruction >> 4 & 0xF) as u8;
    let vy_value = *registers.get_register(vy_index).unwrap();
    let vx_index = (instruction >> 8 & 0xF) as u8;
    let vx_value = *registers.get_register(vx_index).unwrap();
    let dt = *registers.get_delay_timer();
    let i = *registers.get_i_register();
    let active_memory = memory.get_memory();

    match first_nibble {
        // 0 Calls machine code routine at address NNN - not be needed for emulator
        0x0 => {
            // 00E0 - clears screen
            match instruction {
                0x00E0 => {
                    println!("clearing screen");
                    draw.clear()
                }
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
            let nnn = instruction & 0xFFF;
            program_counter.jump(nnn);
        }

        // 2NNN Calls subroutine at NNN
        // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
        0x2 => {
            let pc = program_counter.get_pc() as u16;

            // push current PC to top of stack, which increments the SP
            stack.push(pc);

            // set PC to NNN
            let nnn = instruction & 0xFFF;
            program_counter.jump(nnn)
        }

        // 3xnn - SE Vx, byte
        // Skip next instruction if Vx = nn.
        // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2
        0x3 => {
            let nn = (instruction & 0xFF) as u8;
            // compares
            if nn == vx_value {
                // skip next instruction by incrementing PC by two
                program_counter.increment();
                program_counter.increment();
            }
        }

        // 4XNN Skips the next instruction if VX does not equal NN (usually the next instruction is a jump to skip a code block).
        0x4 => {
            // get NN
            let nn = (instruction & 0xFF) as u8;
            println!("nn: {:02X}, vx_value: {:02X}", nn, vx_value);

            // compares
            if nn != vx_value {
                // skip next instruction by incrementing PC by two
                program_counter.increment();
                program_counter.increment();
            }
        }

        // 5XY0 Skips the next instruction if VX equals VY (usually the next instruction is a jump to skip a code block).
        // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
        0x5 => {
            if vx_value == vy_value {
                program_counter.increment();
                program_counter.increment();
            }
        }

        // 6XNN Sets VX to NN.
        0x6 => {
            let nn = ((instruction as usize) & 0xFF) as u8;
            println!("vx index: {:0X}", vx_index);
            registers.set_register(vx_index, nn);
        }

        // 7XNN Adds NN to VX (carry flag is not changed).
        0x7 => {
            let nn = ((instruction as usize) & 0xFF) as u8;
            // let sum = std::cmp::min(nn as u16 + vx_value as u16, 255) as u8;
            let (sum, _carry) = nn.overflowing_add(vx_value);
            registers.set_register(vx_index, sum);
        }

        0x8 => {
            let instruction_0 = instruction & 0xF;
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
                    if overflow {
                        registers.set_register(0xF, 1);
                    } else {
                        registers.set_register(0xF, 0);
                    }
                    registers.set_register(vx_index, sum);
                }

                5 => {
                    // 8XY5 VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                    let (diff, borrow) = vx_value.overflowing_sub(vy_value);
                    if borrow {
                        registers.set_register(0xF, 0);
                    } else {
                        registers.set_register(0xF, 1);
                    }
                    registers.set_register(vx_index, diff);
                }

                6 => {
                    // 8XY6 Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
                    let vx_lsb = vx_value & 1;
                    registers.set_register(0xF, vx_lsb);
                    registers.set_register(vx_index, vx_value >> 1);
                }

                7 => {
                    // 8XY7 Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not
                    let (diff, borrow) = vy_value.overflowing_sub(vx_value);
                    registers.set_register(vx_index, diff);

                    if borrow {
                        registers.set_register(0xF, 0);
                    } else {
                        registers.set_register(0xF, 1);
                    }
                }

                8 => {
                    // 8XYE	Stores the most significant bit of VX in VF and then shifts VX to the left by 1
                    let vx_msb = (vx_value >> 7) & 1;
                    registers.set_register(0xF, vx_msb);
                    registers.set_register(vx_index, vx_value << 1);
                }
                _ => (),
            }
        }

        // 9XY0 Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump to skip a code block);
        0x9 => {
            if vx_value != vy_value {
                program_counter.increment();
                program_counter.increment();
            }
        }

        // ANNN Sets I to the address NNN.
        0xA => {
            let nnn = instruction & 0xFFF;
            registers.set_i_register(nnn);
        }

        // BNNN Jumps to the address NNN plus V0.
        0xB => {
            let nnn = instruction & 0xFFF;
            stack.jump_to_address(nnn as usize);
        }

        // CXNN Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
        0xC => {
            let nn = (instruction & 0xFF) as u8;
            let random_number = rand::random::<u8>();
            let x = random_number & nn;
            println!(
                "nn: {:02X}, random: {:02X}, vx: {:02X}",
                nn, random_number, x
            );
            registers.set_register(vx_index, x);
        }

        // DXYN Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value does not change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen.
        0xD => {
            let height = (instruction & 0xF) as u8;
            let length = 8 * height as usize;
            let location = *registers.get_i_register() as usize;
            let pixels = &active_memory[location..location + length];
            let dest = &Point {
                x: vx_value as usize,
                y: vy_value as usize,
            };

            draw.blit_raw(pixels, dest, height);
        }

        0xE => {
            let stored_key = vx_value;
            match instruction & 0xFF {
                // EX9E Skips the next instruction if the key stored in VX is pressed (usually the next instruction is a jump to skip a code block).
                0x9E => {
                    if *key_state.key_pressed {
                        match key_state.current_key {
                            Some(value) => {
                                if value as u8 == stored_key {
                                    program_counter.increment();
                                    program_counter.increment();
                                    *key_state.key_pressed = false;
                                }
                            }
                            None => {}
                        }
                    }
                }
                // EXA1 Skips the next instruction if the key stored in VX is not pressed (usually the next instruction is a jump to skip a code block).
                0xA1 => {
                    if *key_state.key_pressed {
                        match key_state.current_key {
                            Some(value) => {
                                if value as u8 != stored_key {
                                    program_counter.increment();
                                    program_counter.increment();
                                    *key_state.key_pressed = false;
                                }
                            }
                            None => {}
                        }
                    }
                }
                _ => {}
            }
        }

        0xF => {
            match instruction & 0xFF {
                // FX07	Sets VX to the value of the delay timer.
                0x07 => {
                    registers.set_register(vx_index, dt);
                }
                // FX0A	A key press is awaited, and then stored in VX (blocking operation, all instruction halted until next key event).
                0x0A => {
                    if *key_state.key_pressed {
                        match key_state.current_key {
                            Some(value) => {
                                println!("key scancode: {}", value);
                                registers.set_register(vx_index, value as u8);
                                *key_state.key_pressed = false;
                            }
                            None => println!("Value is undefined"),
                        }
                    }
                }
                // FX15	Sets the delay timer to VX.
                0x15 => {
                    registers.set_delay_timer(vx_value);
                }
                // FX18	Sets the sound timer to VX.
                0x18 => {
                    // TODO: implement sound
                }
                // FX1E	Adds VX to I. VF is not affected.[c]
                0x1E => {
                    registers.set_i_register(i + vx_value as u16);
                }
                // FX29	Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
                0x29 => {
                    let character_sprite_location = vx_value * 5;
                    registers.set_i_register(character_sprite_location as u16);
                }
                // FX33	Stores the binary-coded decimal representation of VX, with the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
                0x33 => registers.set_i_register(vx_value as u16),
                // FX55	Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
                0x55 => {
                    for (index, hex) in CHAR_SET.iter().enumerate() {
                        let reg_value = registers.get_register(*hex).unwrap();
                        let i = *registers.get_i_register() as usize;
                        active_memory[i + index] = *reg_value;
                    }
                }
                // FX65	Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified.[d]
                0x65 => {
                    for (index, hex) in CHAR_SET.iter().enumerate() {
                        let i = *registers.get_i_register() as usize;
                        let value = active_memory[i + index];
                        registers.set_register(*hex, value);
                    }
                }
                _ => (),
            }
        }
        _ => (),
    }

    // test key press scancode
    // if *key_state.key_pressed {
    //     match key_state.current_key {
    //         Some(value) => {
    //             println!("key scancode: {}", value);
    //             *key_state.key_pressed = false;
    //         }
    //         None => println!("Value is undefined"),
    //     }
    // }
    pixels.render().unwrap();
}

pub fn _decode(_command: bool) -> bool {
    false
}

/* Fetches the program instruction from the chip8 Rom */
pub fn fetch(
    rom: &mut [u8],
    program_counter: &mut ProgramCounter,
    rom_length: usize,
) -> Option<u16> {
    // println!(
    //     "program counter: {}, rom length: {}",
    //     program_counter.get_pc(),
    //     rom_length
    // );
    if program_counter.get_pc() < rom_length - 1 + 512 {
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
    memory: &mut [u8],
    program_counter: &mut ProgramCounter,
    rom_length: usize,
) -> u16 {
    match fetch(memory, program_counter, rom_length) {
        // :04X specifies a width of 4 with leading zeros
        Some(instruction) => {
            // println!("instruction: {:04X}", instruction);
            instruction
        }
        _ => 0x0,
    }
}

/** Prints a font ramp and pixels to demonstrate the edge of the four screen corners */
pub fn _test_print(width: u32, height: u32, screen: &mut [u8], memory: &mut Memory) {
    let mut draw = Draw::new(width, height, screen);

    draw.draw_pixel(&Point { x: 0, y: 0 });
    draw.draw_pixel(&Point { x: 63, y: 0 });
    draw.draw_pixel(&Point { x: 0, y: 31 });
    draw.draw_pixel(&Point { x: 63, y: 31 });

    let font = Font::_new();
    assert_eq!(
        font.get_character(&0x1).unwrap().clone(),
        [0x20, 0x60, 0x20, 0x20, 0x70]
    );

    let mut count = 0;
    let mut y = 3;
    for character in CHAR_SET {
        let mut x = 2 + (count * 5);
        if count == 10 {
            count = 0;
            x = 2 + count * 5;
            y = 10;
        }
        draw.blit_drawable(&Point { x, y }, font.get_font_sprite(&character).unwrap());
        count += 1;
    }

    // This lower portion tests the retrieval of fonts from chip8 memory. this is how it will work in the application when executing instructions.
    let active_memory = memory.get_memory();
    let zero = Font::convert_font_to_sprite(&active_memory[0..5]);
    let zero_sprite = Sprite::new(8, 5, &zero);
    let f = Font::convert_font_to_sprite(&active_memory[75..80]);
    let f_sprite = Sprite::new(8, 5, &f);
    draw.blit_drawable(&Point { x: 20, y: 22 }, &zero_sprite);
    draw.blit_drawable(&Point { x: 28, y: 27 }, &f_sprite);
}

#[derive(Debug)]
pub struct KeyPress<'a> {
    pub current_key: Option<ScanCode>,
    pub key_pressed: &'a mut bool,
}
