use std::time::{Duration, Instant};

use crate::{
    draw::{Draw, Point},
    font,
    memory::{self, Memory},
    program_counter::ProgramCounter,
};
use chip8::{build_pixel_screen, build_window, decode};
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

const INSTRUCTIONS_PER_SECOND: u32 = 700;

pub fn fetch(
    rom: &Vec<u8>,
    program_counter: &mut ProgramCounter,
    rom_length: usize,
) -> Option<u16> {
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

pub fn chip8(width: u32, height: u32, rom: Vec<u8>) {
    let event_loop = EventLoop::new();
    let scale = 20;
    let window = build_window(width * scale, height * scale, &event_loop);

    let mut viewport = build_pixel_screen(&window, width, height).unwrap();
    let screen = viewport.frame_mut();

    ///////
    test_print(width, height, screen);
    ///////

    let time_per_instruction = Duration::from_secs(1) / INSTRUCTIONS_PER_SECOND;

    let rom_length = rom.len();
    println!(
        "rom length: {}, rom [0,1]:  [{}, {}]",
        rom_length, rom[0], rom[1]
    );

    let mut program_counter = ProgramCounter::new();
    let mut _memory = Memory::new();

    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();

        *control_flow = ControlFlow::WaitUntil(start_time + time_per_instruction);

        match event {
            // Event::MainEventsCleared case signifies that all the events which were available up to the point of the last call to the event handler have been processed and the event loop is ready to proceed to the next phase of the loop's body.
            Event::MainEventsCleared => {
                // Run the fetch, decode, and execute cycle here

                // fetch
                match fetch(&rom, &mut program_counter, rom_length) {
                    Some(integer) => {
                        let hex_string = format!("{:X}", integer);
                        if program_counter.get_index() < 10 {
                            println!(
                                "instruction: {}, index: {}",
                                hex_string,
                                program_counter.get_index()
                            );
                        }
                    }
                    _ => (),
                }
                // let command = decode(instruction);
                // execute(command);
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

fn test_print(width: u32, height: u32, screen: &mut [u8]) {
    let mut draw = Draw::new(width, height, screen);

    draw.draw_pixel(0, 0);

    draw.draw_pixel(63, 0);

    draw.draw_pixel(0, 31);

    draw.draw_pixel(63, 31);

    let font = font::Font::new();
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
        // if count == 10 start the second row to print letters A through F
        if count == 10 {
            count = 0;
            x = 2 + count * 5;
            y = 10;
        }
        draw.blit_drawable(&Point { x, y }, font.get_font_sprite(&character).unwrap());
        count += 1;
    }
}
