// use pixels::wgpu::Color;
use pixels::{Error, Pixels, SurfaceTexture};
// use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::ControlFlow;
use winit::window::Window;
use winit::{
    // event::{Event, WindowEvent},
    // event_loop::{ControlFlow, EventLoop},
    event_loop::EventLoop,
    window::WindowBuilder,
};

/// Drawables can be blitted to the pixel buffer and animated.
pub trait Drawable<'a> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &[u8];
}

/// A tiny position vector.
pub struct Point {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

// Display: 64 x 32 pixels (or 128 x 64 for SUPER-CHIP) monochrome, ie. black or white
pub struct Display {
    width: u32,
    event_loop: Box<EventLoop<()>>,
    window: Window,
    viewport: Pixels,
}
impl Display {
    pub fn new(width: u32, height: u32) -> Display {
        let event_loop = Box::new(EventLoop::new());
        let width = width * 10;
        let height = height * 10;
        let window = Self::build_window(width, height, &event_loop);
        let viewport = Self::build_pixel_screen(&window).unwrap();

        Display {
            width,
            event_loop,
            viewport,
            window,
        }
    }

    fn build_window(width: u32, height: u32, event_loop: &EventLoop<()>) -> Window {
        let size = LogicalSize::new((width) as f64, (height) as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    }

    pub fn build_pixel_screen(window: &Window) -> Result<Pixels, Error> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);

        let viewport = Pixels::new(window_size.width, window_size.height, surface_texture)?;
        Ok(viewport)
    }

    pub fn loop_window(self) -> Result<(), pixels::Error> {
        let mut viewport = self.viewport;

        self.event_loop.run(move |event, _, _control_flow| {
            match event {
                // handle RedrawRequested event
                Event::RedrawRequested(_) => {
                    // draw state
                    for (index, pixel) in viewport.frame_mut().iter().enumerate() {
                        if *pixel == 1 {
                            // draw pixel...
                        }
                    }
                }
                // handle other events...
                _ => {}
            }
        });
    }

    // let frame = pixels.frame_mut();

    // for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
    //     let x = (i % self.width as usize) as i16;
    //     let y = (i / self.width as usize) as i16;

    //     if x > 100 && x < 200 && y > 200 && y < 400 {
    //         pixel[0] = 0xE2; // R : E2 = (14 * 16^1) + (2 * 16^0) = 224 + 2 = 226
    //         pixel[1] = 0x1B; // G : 1B = (1 * 16^1) + (11 * 16^0) = 16 + 11 = 27
    //         pixel[2] = 0x88; // B : 88 = (8 * 16^1) + (8 * 16^0) = 128 + 8 = 136
    //         pixel[3] = 0xff; // A : ff = (15 * 16^1) + (15 * 16^0) = 240 + 15 = 255
    //     }
    // }

    // // Draw it to the `SurfaceTexture`
    // pixels.render().unwrap();

    // TODO: change to draw function with Drawable
    // pub fn draw<E>(self, dest: &Point, _element: &[u8; 5])
    // where
    //     E: Drawable,
    pub fn draw(&mut self, x: usize, y: usize) {
        // assert!(dest.x <= self.width as usize);
        // assert!(dest.y <= self.height as usize);
        let viewport = &mut self.viewport;

        let pixels = viewport.frame_mut();

        let i = (y * self.width as usize + x) as usize;

        for (index, pixel) in pixels.chunks_exact_mut(4).enumerate() {
            if index == i {
                pixel.copy_from_slice(&[0xE2, 0x1B, 0x88, 0xff])
            }
        }

        viewport.render().unwrap();

        self.window.request_redraw();
    }
}
