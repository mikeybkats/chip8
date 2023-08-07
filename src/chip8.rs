use crate::{
    draw::{Draw, Point},
    font,
};
use chip8::{build_pixel_screen, build_window};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

pub fn chip8(width: u32, height: u32) {
    let event_loop = EventLoop::new();
    let scale = 20;
    let window = build_window(width * scale, height * scale, &event_loop);
    let mut viewport = build_pixel_screen(&window, width, height).unwrap();
    let screen = viewport.frame_mut();

    test_print(width, height, screen);

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
                // self.redraw();
                viewport.render().unwrap();
            }
            // handle other events...
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
