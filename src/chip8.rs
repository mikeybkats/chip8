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

    let sprite = font.get_font_sprite(&'0').unwrap();
    println!("font sprite: {:?}", sprite);

    display.blit_drawable(&Point { x: 0, y: 0 }, sprite);

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
