use pixels::Pixels;
use winit::event::{ScanCode, ElementState};

use crate::{
    draw::{Draw, Point},
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
    key_state: KeyPress,
) {
    // println!("{:04X}", instruction);
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

    let mut draw = Draw::new(width, screen);

    let vy_index = (instruction >> 4 & 0xF) as u8;
    let vy_value = *registers.get_register(vy_index).unwrap();
    let vx_index = (instruction >> 8 & 0xF) as u8;
    let vx_value = *registers.get_register(vx_index).unwrap();
    let dt = *registers.get_delay_timer();
    let i = *registers.get_i_register();
    let active_memory = memory.get_memory();

    let mut render = false;

    match first_nibble {
        // 0 Calls machine code routine at address NNN - not be needed for emulator
        0x0 => {
            match instruction & 0xFF {
                // 00E0 - clears screen
                0xE0 => {
                    draw.clear();
                    render = true
                },
                // 00EE
                // Return from a subroutine.
                // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
                0xEE => {
                    let current_address = stack.pop();
                    program_counter.jump(current_address);
                }
                _ => (),
            }
        }

        // 1NNN Jumps to address at NNN
        // The interpreter sets the program counter to nnn.
        0x1 => {
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
                program_counter.increment_by(2);
            }
        }

        // 4XNN Skips the next instruction if VX does not equal NN (usually the next instruction is a jump to skip a code block).
        0x4 => {
            // get NN
            let nn = (instruction & 0xFF) as u8;

            // compares
            if nn != vx_value {
                // skip next instruction by incrementing PC by two
                program_counter.increment_by(2);
            }
        }

        // 5XY0 Skips the next instruction if VX equals VY (usually the next instruction is a jump to skip a code block).
        // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
        0x5 => {
            if vx_value == vy_value {
                program_counter.increment_by(2);
            }
        }

        // 6XNN Sets VX to NN.
        0x6 => {
            let nn = ((instruction as usize) & 0xFF) as u8;
            registers.set_register(vx_index, nn);
        }

        // 7XNN Adds NN to VX (carry flag is not changed).
        0x7 => {
            let nn = ((instruction as usize) & 0xFF) as u8;
            let (sum, _carry) = nn.overflowing_add(vx_value);
            // if carry {
            //     registers.set_register(vx_index, 255);
            // } else {
                registers.set_register(vx_index, sum);
            // }
        }

        0x8 => {
            let instruction_0 = instruction & 0xF;
            match instruction_0 {
                0x0 => {
                    // 8XY0 Sets VX to the value of VY.
                    registers.set_register(vx_index, vy_value);
                }
                0x1 => {
                    // 8XY1 Sets VX to VX or VY. (bitwise OR operation)
                    registers.set_register(vx_index, vx_value | vy_value);

                    // reset flag register to zero
                    registers.set_register(0xF, 0);
                }

                0x2 => {
                    // 8XY2 Sets VX to VX and VY. (bitwise AND operation)
                    registers.set_register(vx_index, vx_value & vy_value);
                
                    // reset flag register to zero
                    registers.set_register(0xF, 0);
                }

                0x3 => {
                    // 8XY3 Sets VX to VX xor VY.
                    registers.set_register(vx_index, vx_value ^ vy_value);

                    // reset flag register to zero
                    registers.set_register(0xF, 0);
                }

                0x4 => {
                    // 8XY4 Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
                    // fourth checkmark: if vF can be used as the vX input  
                    let (sum, overflow) = vy_value.overflowing_add(vx_value);
                    registers.set_register(vx_index, sum);

                    if overflow {
                        registers.set_register(0xF, 1);
                    } else {
                        registers.set_register(0xF, 0);
                    }
                }

                0x5 => {
                    // 8XY5 VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                    let (diff, borrow) = vx_value.overflowing_sub(vy_value);
                    registers.set_register(vx_index, diff);
                    
                    if borrow {
                        registers.set_register(0xF, 0);
                    } else {
                        registers.set_register(0xF, 1);
                    }
                }

                0x6 => {
                    // 8XY6 Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
                    // failing on check if vF can be used as the vY input 
                    registers.set_register(vx_index, vx_value >> 1);
                    registers.set_register(0xF, vx_value & 1);
                }

                0x7 => {
                    // 8XY7 Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not
                    let (diff, borrow) = vy_value.overflowing_sub(vx_value);
                    registers.set_register(vx_index, diff);

                    if borrow {
                        registers.set_register(0xF, 0);
                    } else {
                        registers.set_register(0xF, 1);
                    }
                }

                0xE => {
                    // 8XYE	Stores the most significant bit of VX in VF and then shifts VX to the left by 1
                    registers.set_register(vx_index, vx_value << 1);
                    registers.set_register(0xF, vx_value >> 7 & 1); 
                }
                _ => (),
            }
        }

        // 9XY0 Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump to skip a code block);
        0x9 => {
            if vx_value != vy_value {
                program_counter.increment_by(2);
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
            program_counter.jump(nnn + *registers.get_register(0).unwrap() as u16);
        }

        // CXNN Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
        0xC => {
            let nn = (instruction & 0xFF) as u8;
            let random_number = rand::random::<u8>();
            let x = random_number & nn;
            registers.set_register(vx_index, x);
        }

        // DXYN Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value does not change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen.
        0xD => {
            let height = (instruction & 0xF) as u8;
            let length = 8 * height as usize;
            let location_u16 = *registers.get_i_register();
            let location = location_u16 as usize;
            let pixels = &active_memory[location..location + length];
            let dest = &Point {
                x: (vx_value % 64) as usize,
                y: (vy_value % 32) as usize,
            };
            registers.set_register(0xF, 0);

            let set_flag_register = draw.blit_raw(pixels, dest, height);

            if set_flag_register {
                registers.set_register(0xF, 1);
            } else {
                registers.set_register(0xF, 0);
            }

            render = true;

        }

        0xE => {
            // let stored_key = vx_value;
            match instruction & 0xFF {
                // EX9E Skips the next instruction if the key stored in VX is pressed (usually the next instruction is a jump to skip a code block).
                0x9E => {
                    match key_state.current_key {
                        Some(value) => {
                            if value as u8 == vx_value {
                                program_counter.increment_by(2);
                            }
                        }
                        None => {}
                    }
                }
                // EXA1 Skips the next instruction if the key stored in VX is not pressed (usually the next instruction is a jump to skip a code block).
                0xA1 => {
                    match key_state.current_key {
                        Some(value) => {
                            if value as u8 != vx_value {
                                program_counter.increment_by(2);
                            }

                        }
                        None => {
                            program_counter.increment_by(2);
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
                    match key_state.state {
                        Some(ElementState::Released) => {
                            match key_state.current_key {
                                Some(value) => {
                                    registers.set_register(vx_index, value as u8);
                                }
                                _ => {
                                    program_counter.decrement();
                                    program_counter.decrement();
                                },
                            }
                        },
                        _ => {}
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
                0x33 => { 
                    let vx_hundreds = vx_value / 100;
                    let vx_tens = (vx_value / 10) % 10;
                    let vx_ones = vx_value % 10;

                    let i = *registers.get_i_register() as usize;

                    active_memory[i] = vx_hundreds;
                    active_memory[i+1] = vx_tens;
                    active_memory[i+2] = vx_ones;
                }
                    ,
                // FX55	Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
                0x55 => {
                    for value in 0..vx_index + 1 {
                        let reg_value = *registers.get_register(value).unwrap();
                        let i = *registers.get_i_register() as usize;

                        active_memory[i + value as usize] = reg_value;

                        // incrementing i register for older games
                        // registers.set_i_register((i + value as usize) as u16);
                    }
                }
                // FX65	Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified.[d]
                0x65 => {
                    for value in 0..vx_index + 1 {
                        let i = *registers.get_i_register() as usize;

                        let mem_value = active_memory[i + value as usize];

                        registers.set_register(value, mem_value);

                        // incrementing i register for older games
                        // registers.set_i_register((i + value as usize) as u16);
                    }
                }
                _ => (),
            }
        }
        _ => (),
    }

    if render {
        pixels.render().unwrap();
    }
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
        Some(instruction) => {
            instruction
        }
        _ => 0x0,
    }
}

pub fn match_key(key_scancode: u32) -> Option<u32>{
    match key_scancode {
        // Key 1 - 18: 0x1
        18 => {
            Some(0x1)
        }
        // Key 2 - 19: 0x2
        19 => {
            Some(0x2)
        }
        // Key 3 - 20: 0x3
        20 => {
            Some(0x3)
        }
        // Key 4 - 21: 0xC
        21 => {
            Some(0xC)
        }
        // Key Q - 12: 0x4
        12 => {
            Some(0x4)
        }
        // Key W - 13: 0x5
        13 => {
            Some(0x5)
        }
        // Key E - 14: 0x6
        14 => {
            Some(0x6)
        }
        // Key R - 15: 0xD
        15 => {
            Some(0xD)
        }
        // Key A - 0:  0x7
        0 => {
            Some(0x7)
        }
        // Key S - 1:  0x8
        1 => {
            Some(0x8)
        }
        // Key D - 2:  0x9
        2 => {
            Some(0x9)
        }
        // Key F - 3:  0xE
        3 => {
            Some(0xE)
        }
        // Key Z - 6:  0xA
        6 => {
            Some(0xA)
        }
        // Key X - 7:  0x0
        7 => {
            Some(0x0)
        }
        // Key C - 8:  0xB
        8 => {
            Some(0xB)
        }
        // Key V - 9:  0xF
        9 => {
            Some(0xF)
        }
        _ => {
            None
        }
    }
}

#[derive(Debug)]
pub struct KeyPress {
    pub current_key: Option<ScanCode>,
    pub state: Option<ElementState>,
}
