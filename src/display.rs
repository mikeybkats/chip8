use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::window::Window;
use winit::{event_loop::EventLoop, window::WindowBuilder};

/// Drawables can be blitted to the pixel buffer and animated.
pub trait Drawable<'a> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &[u8];
}

/// A tiny position vector.
// pub struct Point {
//     pub(crate) x: usize,
//     pub(crate) y: usize,
// }

// Display: 64 x 32 pixels (or 128 x 64 for SUPER-CHIP) monochrome, ie. black or white
pub struct Display {
    pub window: Window,
    viewport: Pixels,
    width: u32,
}
impl Display {
    pub fn new(width: u32, height: u32, event_loop: &EventLoop<()>) -> Display {
        let scale = 20;
        let window = Self::build_window(width * scale, height * scale, &event_loop);
        let viewport = Self::build_pixel_screen(&window, width, height).unwrap();

        Display {
            viewport,
            window,
            width,
        }
    }

    /*
     * redraw
     */
    pub fn redraw(&self) {
        self.viewport.render().unwrap();
    }

    /*
     * build_window
     * builds the window for rendering pixels
     */
    fn build_window(width: u32, height: u32, event_loop: &EventLoop<()>) -> Window {
        let size = LogicalSize::new((width) as f64, (height) as f64);
        WindowBuilder::new()
            .with_title("chip 8")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    }

    /*
     * build_pixel_screen
     * builds the pixel screen - a canvas for rendering pixels
     */
    pub fn build_pixel_screen(window: &Window, width: u32, height: u32) -> Result<Pixels, Error> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);

        let viewport = Pixels::new(width, height, surface_texture)?;
        Ok(viewport)
    }

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
                println!("Copying to slice");
                // pixel[i..i + 4].copy_from_slice(&[0xE2, 0x1B, 0x88, 0xff]);
                pixel[0..4].copy_from_slice(&[0xE2, 0x1B, 0x88, 0xff]);
            }
        }
    }

    // blit is shorthand for bit block transfer
    // it refers to the operation of copying a block of data to a block of pixels in memory
    // pub fn blit_drawable<'a, E>(pixels: &mut [u8], dest: &Point, _element: &'a E)
    // where
    //     E: Drawable<'a>,
    // {
    // }
}
