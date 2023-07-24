use crate::Display;
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

    chip8_program_loop(event_loop);
}

fn chip8_program_loop(event_loop: EventLoop<()>) {
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
            }
            // handle other events...
            _ => {}
        }
    });
}
