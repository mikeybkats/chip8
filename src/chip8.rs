use std::time::{Duration, Instant};

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    display::{build_pixels, build_window},
    draw::{Draw, Point},
    emulator::{execute, fetch_instruction, test_print},
    program_counter::ProgramCounter,
    registers::Registers,
    stack::Stack,
};

// extern "C" {
//     static mut MEMORY: Memory;
// }

pub fn chip8(width: u32, height: u32, rom: Vec<u8>) {
    let event_loop = EventLoop::new();
    let scale = 20;
    let window = build_window(width * scale, height * scale, &event_loop);
    let mut pixels = build_pixels(&window, width, height).unwrap();

    const INSTRUCTIONS_PER_SECOND: u32 = 700;
    let time_per_instruction = Duration::from_secs(1) / INSTRUCTIONS_PER_SECOND;

    let mut stack = Stack::new();
    let mut program_counter = ProgramCounter::new();
    let mut registers = Registers::new();
    let screen = pixels.frame_mut();

    // ///////
    test_print(width, height, screen);
    // ///////

    // main event loop
    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();

        *control_flow = ControlFlow::WaitUntil(start_time + time_per_instruction);

        match event {
            // Event::MainEventsCleared case signifies that all the events which were available up to the point of the last call to the event handler have been processed and the event loop is ready to proceed to the next phase of the loop's body.
            Event::MainEventsCleared => {
                // fetch
                let rom_length = rom.len();
                let instruction = fetch_instruction(&rom, &mut program_counter, rom_length);
                execute(
                    instruction,
                    &mut stack,
                    &mut registers,
                    &mut program_counter,
                    &mut pixels,
                    width,
                    height,
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
                            virtual_keycode: Some(virtual_keycode),
                            ..
                        },
                    ..
                } => {
                    println!("Key pressed: {:?}", virtual_keycode);
                    if virtual_keycode == VirtualKeyCode::C {
                        let screen = pixels.frame_mut();
                        let mut draw = Draw::new(width, height, screen);
                        draw.clear();
                    }
                }
                _ => {}
            },
            _ => {}
        }
    });
}
