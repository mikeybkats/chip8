use std::{
    thread,
    time::{Duration, Instant},
};

use crate::{
    draw::{Draw, Point},
    font,
};
use chip8::{build_pixel_screen, build_window};
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::EventLoop,
};

const INSTRUCTIONS_PER_SECOND: u32 = 700;

fn fetch() -> bool {
    println!("Fetching");
    false
}
fn decode() -> bool {
    println!("Decoding");
    false
}
fn execute() -> bool {
    println!("Executing");
    false
}

pub fn chip8(width: u32, height: u32) {
    let event_loop = EventLoop::new();
    let scale = 20;
    let window = build_window(width * scale, height * scale, &event_loop);

    let mut viewport = build_pixel_screen(&window, width, height).unwrap();
    let screen = viewport.frame_mut();

    test_print(width, height, screen);

    event_loop.run(move |event, _, control_flow| {
        let delay = Duration::from_secs(1) / INSTRUCTIONS_PER_SECOND;

        match event {
            Event::MainEventsCleared => {
                // Run the fetch, decode, and execute cycle here
                // for _ in 0..INSTRUCTIONS_PER_SECOND {
                fetch();
                decode();
                execute();
                // }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            }
            Event::RedrawRequested(_) => {
                println!("Redrawing");
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

        let start_time = Instant::now();

        let elapsed_time = start_time.elapsed();
        if elapsed_time < delay {
            thread::sleep(delay - elapsed_time);
        } else {
            println!("Took longer than expected to process instructions!");
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
