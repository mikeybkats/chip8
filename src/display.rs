// use pixels::wgpu::Color;
use pixels::{Error, Pixels, SurfaceTexture};
// use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
// use winit::event::Event;
// use winit::event_loop::ControlFlow;
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
    event_loop: EventLoop<()>,
    window: Window,
    viewport: Pixels,
}
impl Display {
    pub fn new(width: u32, height: u32) -> Display {
        let scale = 20;
        let event_loop = EventLoop::new();
        let window = Self::build_window(width * scale, height * scale, &event_loop);
        let viewport = Self::build_pixel_screen(&window, width, height).unwrap();

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

    pub fn build_pixel_screen(window: &Window, width: u32, height: u32) -> Result<Pixels, Error> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);

        let viewport = Pixels::new(width, height, surface_texture)?;
        Ok(viewport)
    }

    pub fn loop_window(self) -> Result<(), pixels::Error> {
        // let mut viewport = self.viewport;

        self.event_loop.run(move |event, _, _control_flow| {
            match event {
                // handle RedrawRequested event
                // Event::RedrawRequested(_) => {
                //     // draw state
                //     for (index, pixel) in viewport.frame_mut().iter().enumerate() {
                //         if *pixel == 1 {
                //             // draw pixel...
                //         }
                //     }
                // }
                // handle other events...
                _ => {}
            }
        });
    }

    // TODO: implement draw function with Drawable
    // pub fn draw<E>(self, dest: &Point, _element: &[u8; 5])
    // where
    //     E: Drawable,

    /*
     * draw_pixel
     * draws a pixel at the x, y co-ordinates
     */
    pub fn draw_pixel(&mut self, x: usize, y: usize) {
        let viewport = &mut self.viewport;

        let pixels = viewport.frame_mut();

        let base_point = (y * (self.width as usize)) + x as usize;

        for (i, pixel) in pixels.chunks_exact_mut(4).enumerate() {
            if i == base_point {
                println!("base point: {}", base_point);
                pixel[0..4].copy_from_slice(&[0xE2, 0x1B, 0x88, 0xff]);
            }
        }

        viewport.render().unwrap();

        self.window.request_redraw();
    }
}
