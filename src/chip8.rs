use std::time::{Duration, Instant};

use winit::{
    event::{ElementState, Event, KeyboardInput, ScanCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    display::{build_pixels, build_window},
    emulator::{execute, fetch_instruction, match_key, KeyPress},
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
    let mut memory = Memory::new();
    memory.set_rom(&rom).unwrap();
    memory.set_fonts();

    let mut current_key: Option<ScanCode> = None;
    let mut key_pressed: Option<ElementState> = None;

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
                    state: key_pressed,
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
                    current_key = match_key(key_scancode);
                    key_pressed = Some(ElementState::Pressed);
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(_key),
                            ..
                        },
                    ..
                } => {
                    current_key = None;
                    key_pressed = Some(ElementState::Released);
                }
                _ => {}
            },
            _ => {}
        }
    });
}
