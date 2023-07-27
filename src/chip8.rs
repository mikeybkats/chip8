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

    let sprite_0 = font.get_font_sprite(&'0').unwrap();
    display.blit_drawable(&Point { x: 2, y: 3 }, sprite_0);

    let sprite_1 = font.get_font_sprite(&'1').unwrap();
    display.blit_drawable(&Point { x: 9, y: 3 }, sprite_1);

    let sprite_2 = font.get_font_sprite(&'2').unwrap();
    display.blit_drawable(&Point { x: 16, y: 3 }, sprite_2);

    let sprite_3 = font.get_font_sprite(&'3').unwrap();
    display.blit_drawable(&Point { x: 22, y: 3 }, sprite_3);

    let sprite_4 = font.get_font_sprite(&'4').unwrap();
    display.blit_drawable(&Point { x: 29, y: 3 }, sprite_4);

    let sprite_5 = font.get_font_sprite(&'5').unwrap();
    display.blit_drawable(&Point { x: 36, y: 3 }, sprite_5);

    let sprite_6 = font.get_font_sprite(&'6').unwrap();
    display.blit_drawable(&Point { x: 43, y: 3 }, sprite_6);

    let sprite_7 = font.get_font_sprite(&'7').unwrap();
    display.blit_drawable(&Point { x: 50, y: 3 }, sprite_7);

    let sprite_8 = font.get_font_sprite(&'8').unwrap();
    display.blit_drawable(&Point { x: 56, y: 3 }, sprite_8);

    let sprite_9 = font.get_font_sprite(&'9').unwrap();
    display.blit_drawable(&Point { x: 2, y: 13 }, sprite_9);

    let sprite_a = font.get_font_sprite(&'A').unwrap();
    display.blit_drawable(&Point { x: 9, y: 13 }, sprite_a);

    let sprite_b = font.get_font_sprite(&'B').unwrap();
    display.blit_drawable(&Point { x: 16, y: 13 }, sprite_b);

    let sprite_c = font.get_font_sprite(&'C').unwrap();
    display.blit_drawable(&Point { x: 22, y: 13 }, sprite_c);

    let sprite_d = font.get_font_sprite(&'D').unwrap();
    display.blit_drawable(&Point { x: 29, y: 13 }, sprite_d);

    let sprite_e = font.get_font_sprite(&'E').unwrap();
    display.blit_drawable(&Point { x: 36, y: 13 }, sprite_e);

    let sprite_f = font.get_font_sprite(&'F').unwrap();
    display.blit_drawable(&Point { x: 43, y: 13 }, sprite_f);

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
