use std::time::{Duration, Instant};

use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    memory::Memory,
    program_counter::ProgramCounter,
    utils::{build_pixel_screen, build_window, decode, execute, fetch_instruction, test_print},
};

pub fn chip8(width: u32, height: u32, rom: Vec<u8>) {
    const INSTRUCTIONS_PER_SECOND: u32 = 700;
    let event_loop = EventLoop::new();
    let scale = 20;
    let window = build_window(width * scale, height * scale, &event_loop);
    let mut viewport = build_pixel_screen(&window, width, height).unwrap();
    let screen = viewport.frame_mut();
    let time_per_instruction = Duration::from_secs(1) / INSTRUCTIONS_PER_SECOND;
    let rom_length = rom.len();
    let mut program_counter = ProgramCounter::new();

    // TODO: implement memory and stack pointer
    let memory = Memory::new();
    let memory_rom = memory.set_rom(&rom).unwrap();

    ///////
    test_print(width, height, screen);
    ///////

    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();

        *control_flow = ControlFlow::WaitUntil(start_time + time_per_instruction);

        match event {
            // Event::MainEventsCleared case signifies that all the events which were available up to the point of the last call to the event handler have been processed and the event loop is ready to proceed to the next phase of the loop's body.
            Event::MainEventsCleared => {
                // fetch
                let instruction = fetch_instruction(&memory_rom, &mut program_counter, rom_length);

                // decode
                let command = decode(instruction);

                // execute
                execute(command);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            }
            Event::RedrawRequested(_) => {
                viewport.render().unwrap();
            }
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(virtual_keycode),
                            ..
                        },
                    ..
                } => {
                    println!("Key pressed: {:?}", virtual_keycode);
                }
                _ => {}
            },
            _ => {}
        }
    });
}
