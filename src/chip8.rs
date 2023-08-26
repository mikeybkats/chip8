use std::time::{Duration, Instant};

use winit::{
    event::{ElementState, Event, KeyboardInput, ScanCode, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    display::{build_pixels, build_window},
    draw::Draw,
    emulator::{execute, fetch_instruction, test_print},
    program_counter::ProgramCounter,
    registers::Registers,
    stack::Stack,
};

#[derive(Debug)]
pub struct KeyPress<'a> {
    pub current_key: Option<ScanCode>,
    pub key_pressed: &'a mut bool,
}

// pub struct DelayTimer<'a> {
//     delay_timer: &'a mut u8,
//     delay_timer_written: Instant,
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
    let mut current_key: Option<ScanCode> = None;
    let mut key_pressed: bool = false;

    // ///////
    test_print(width, height, screen);
    // ///////

    // main event loop
    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();
        *control_flow = ControlFlow::WaitUntil(start_time + time_per_instruction);

        // TODO: how to correctly implement this delay timer?
        // Instead, figure out a way to implement this in a single thread. I'd recommend one of two approaches:

        // 1. Keep track of the time the last value was written to the timer register. When reading from the register, calculate how long ago it was written to, and subtract an appropriate value from the result.

        // 2. Keep track of how many instructions are being executed overall, and subtract 1 from the timer register every N instructions, where N is a large number such that N instructions is about 1/60 second.
        // if *registers.get_delay_register() != 0 {
        //     if delay_timer == 0 {
        //         registers.set_delay_register(0);
        //         delay_timer = 60;
        //     } else {
        //         delay_timer -= 1;
        //         println!("delay timer: {}", delay_timer);
        //         let elapsed_time = start_time.elapsed();
        //         if elapsed_time < delay {
        //             // this does not work because it halts the program
        //             thread::sleep(delay - elapsed_time);
        //         } else {
        //             println!("Loop took longer than expected!");
        //         }
        //     }
        // }

        match event {
            // Event::MainEventsCleared case signifies that all the events which were available up to the point of the last call to the event handler have been processed and the event loop is ready to proceed to the next phase of the loop's body.
            Event::MainEventsCleared => {
                // fetch
                let rom_length = rom.len();
                let instruction = fetch_instruction(&rom, &mut program_counter, rom_length);

                let key_state = KeyPress {
                    current_key,
                    key_pressed: &mut key_pressed,
                };

                execute(
                    instruction,
                    &mut stack,
                    &mut registers,
                    &mut program_counter,
                    &mut pixels,
                    width,
                    height,
                    &rom,
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
                            virtual_keycode: Some(virtual_keycode),
                            scancode: key_scancode,
                            ..
                        },
                    ..
                } => {
                    current_key = Some(key_scancode);
                    // println!("scancode loop: {:?}", current_key);
                    key_pressed = true;

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
