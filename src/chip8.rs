use crate::{display::Point, font, Display};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

pub fn chip8(width: u32, height: u32) {
    let event_loop = EventLoop::new();

    let mut display = Display::new(width as u32, height as u32, &event_loop);

    display.draw_pixel(0, 0);

    display.draw_pixel(63, 0);

    display.draw_pixel(0, 31);

    display.draw_pixel(63, 31);

    let font = font::Font::new();
    assert_eq!(
        font.get_character(&'1').unwrap().clone(),
        &[0x20, 0x60, 0x20, 0x20, 0x70]
    );

    // println!("font sprite: {:?}", sprite);

    // let sprite_0 = font.get_font_sprite(&'0').unwrap();
    // display.blit_drawable(&Point { x: 2, y: 3 }, sprite_0);

    let sprite_1 = font.get_font_sprite(&'1').unwrap();
    // display.blit_drawable(&Point { x: 12, y: 11 }, sprite_1);

    // let sprite_2 = font.get_font_sprite(&'2').unwrap();
    // display.blit_drawable(&Point { x: 19, y: 3 }, sprite_2);

    // let sprite_3 = font.get_font_sprite(&'3').unwrap();
    // display.blit_drawable(&Point { x: 27, y: 3 }, sprite_3);

    chip8_program_loop(event_loop, display);
}

fn chip8_program_loop(event_loop: EventLoop<()>, display: Display) {
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            }
            // handle RedrawRequested event
            Event::RedrawRequested(_) => {
                // redraw state
                println!("Redrawing");
                display.redraw();
            }
            // handle other events...
            _ => {}
        }
    });
}
