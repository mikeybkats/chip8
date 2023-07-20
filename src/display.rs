use pixels::wgpu::Color;
use pixels::{Error, Pixels, SurfaceTexture};
// use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
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
    height: u32,
    event_loop: EventLoop<()>,
    window: Window,
}
impl Display {
    pub fn new(width: u32, height: u32) -> Display {
        let event_loop = EventLoop::new();
        let width = width * 10;
        let height = height * 10;
        let window = Self::build_window(width, height, &event_loop);

        Display {
            width,
            height,
            event_loop,
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

    pub fn pixels(&self) -> Result<Pixels, Error> {
        let window_size = self.window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &self.window);
        Pixels::new(window_size.width, window_size.height, surface_texture)
    }

    pub fn loop_window(self, scope: ()) -> Result<(), pixels::Error> {
        let mut pixels = self.pixels().unwrap();

        self.event_loop.run(move |_event, _, _control_flow| {
            let frame = pixels.frame_mut();

            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let x = (i % self.width as usize) as i16;
                let y = (i / self.width as usize) as i16;

                if x > 100 && x < 200 && y > 200 && y < 400 {
                    pixel[0] = 0xE2; // R : E2 = (14 * 16^1) + (2 * 16^0) = 224 + 2 = 226
                    pixel[1] = 0x1B; // G : 1B = (1 * 16^1) + (11 * 16^0) = 16 + 11 = 27
                    pixel[2] = 0x88; // B : 88 = (8 * 16^1) + (8 * 16^0) = 128 + 8 = 136
                    pixel[3] = 0xff; // A : ff = (15 * 16^1) + (15 * 16^0) = 240 + 15 = 255
                }
            }

            // Draw it to the `SurfaceTexture`
            pixels.render().unwrap();

            self.window.request_redraw();
        });
    }

    // TODO: change to draw function with Drawable
    pub fn draw_font<E>(self, dest: &Point, element: &[u8; 5])
    // where
    //     E: Drawable,
    {
        assert!(dest.x <= self.width as usize);
        assert!(dest.y <= self.height as usize);

        let mut pixels = self.pixels().unwrap();

        let screen = pixels.frame_mut();

        let mut s = 0;
        for y in 0..5 {
            // let i = dest.x * 4 + dest.y * self.width as usize * 4 + y * self.width as usize * 4;
            // let zipped = screen[i..i + self.width as usize]
            //     .iter_mut()
            //     .zip(&pixels[s..s + self.width as usize]);
        }
    }
}
