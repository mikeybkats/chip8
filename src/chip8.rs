use std::time::{Duration, Instant};

use winit::{
    event::{ElementState, Event, KeyboardInput, ScanCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    display::{build_pixels, build_window},
    emulator::{execute, fetch_instruction, KeyPress},
    memory::Memory,
    program_counter::ProgramCounter,
    registers::Registers,
    stack::Stack,
};

pub fn chip8(width: u32, height: u32, rom: Vec<u8>) {
    let event_loop = EventLoop::new();
    let scale = 20;
    let window = build_window(width * scale, height * scale, &event_loop);
    let mut pixels = build_pixels(&window, width, height).unwrap();

    const INSTRUCTIONS_PER_SECOND: u32 = 700;
    let time_per_instruction = Duration::from_secs(1) / INSTRUCTIONS_PER_SECOND;

    let mut stack = Stack::new();
    let mut program_counter = ProgramCounter::new();
    program_counter.set_counter(512);
    let mut registers = Registers::new();
    let mut current_key: Option<ScanCode> = None;
    let mut key_pressed: bool = false;
    let mut memory = Memory::new();
    memory.set_rom(&rom).unwrap();
    memory.set_fonts();

    // main event loop
    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();
        *control_flow = ControlFlow::WaitUntil(start_time + time_per_instruction);

        match event {
            // Event::MainEventsCleared case signifies that all the events which were available up to the point of the last call to the event handler have been processed and the event loop is ready to proceed to the next phase of the loop's body.
            Event::MainEventsCleared => {
                // fetch
                let rom_length = rom.len();
                let instruction =
                    fetch_instruction(memory.get_memory(), &mut program_counter, rom_length);

                let key_state = KeyPress {
                    current_key,
                    key_pressed: &mut key_pressed,
                };

                execute(
                    instruction,
                    &mut memory,
                    &mut stack,
                    &mut registers,
                    &mut program_counter,
                    &mut pixels,
                    width,
                    key_state,
                );
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            }
            Event::RedrawRequested(_) => {
                println!("redrawing");
                pixels.render().unwrap();
            }
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(_virtual_keycode),
                            scancode: key_scancode,
                            ..
                        },
                    ..
                } => {
                    // current_key = Some(key_scancode);

                    match key_scancode {
                        // 1 - 18: 0x1
                        18 => {
                            current_key = Some(0x1);
                        }
                        // 2 - 19: 0x2
                        19 => {
                            current_key = Some(0x2);
                        }
                        // 3 - 20: 0x3
                        20 => {
                            current_key = Some(0x3);
                        }
                        // 4 - 21: 0xC
                        21 => {
                            current_key = Some(0xC);
                        }
                        // 12: 0x4
                        12 => {
                            current_key = Some(0x4);
                        }
                        // 13: 0x5
                        13 => {
                            current_key = Some(0x5);
                        }
                        // 14: 0x6
                        14 => {
                            current_key = Some(0x6);
                        }
                        // 15: 0xD
                        15 => {
                            current_key = Some(0xD);
                        }
                        // 0:  0x7
                        0 => {
                            current_key = Some(0x7);
                        }
                        // 1:  0x8
                        1 => {
                            current_key = Some(0x8);
                        }
                        // 2:  0x9
                        2 => {
                            current_key = Some(0x9);
                        }
                        // 3:  0xE
                        3 => {
                            current_key = Some(0xE);
                        }
                        // 6:  0xA
                        6 => {
                            current_key = Some(0xA);
                        }
                        // 7:  0x0
                        7 => {
                            current_key = Some(0x0);
                        }
                        // 8:  0xB
                        8 => {
                            current_key = Some(0xB);
                        }
                        // 9:  0xF
                        9 => {
                            current_key = Some(0xF);
                        }
                        _ => {
                            current_key = Some(0x0);
                        }
                    }
                    println!(
                        "scancode: {:?}, current key: {:0X}",
                        key_scancode,
                        current_key.unwrap()
                    );

                    key_pressed = true;

                    // if virtual_keycode == VirtualKeyCode::C {
                    //     let screen = pixels.frame_mut();
                    //     let mut draw = Draw::new(width, screen);
                    //     draw.clear();
                    // }
                }
                _ => {}
            },
            _ => {}
        }
    });
}
