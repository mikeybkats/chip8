use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::window::Window;
use winit::{event_loop::EventLoop, window::WindowBuilder};

// Display: 64 x 32 pixels (or 128 x 64 for SUPER-CHIP) monochrome, ie. black or white
pub struct Display {
    pub window: Window,
    viewport: Pixels,
    width: usize,
    height: usize,
}
impl Display {
    pub fn new(width: u32, height: u32, event_loop: &EventLoop<()>) -> Display {
        let scale = 20;
        let window = Self::build_window(width * scale, height * scale, &event_loop);
        let viewport = Self::build_pixel_screen(&window, width, height).unwrap();

        Display {
            viewport,
            window,
            width: width as usize,
            height: height as usize,
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
                pixel[0..4].copy_from_slice(&[0xE2, 0x1B, 0x88, 0xff]);
            }
        }

        // self.window.request_redraw(); // this method does not seem to be needed for some reason
    }

    // blit is shorthand for bit block transfer
    // it refers to the operation of copying a block of data to a block of pixels in memory
    pub fn blit_drawable<'a, E>(&mut self, dest: &Point, sprite: &E)
    where
        E: Drawable,
    {
        assert!(dest.x + sprite.width() <= self.width);
        assert!(dest.y + sprite.height() <= self.height);

        // get viewport
        let viewport = &mut self.viewport;
        // get the pixels_screen
        let pixels_screen = viewport.frame_mut();

        // calculate the base point: where to draw the sprite
        let mut draw_point = (dest.y * self.width * 4) + dest.x;

        let mut count = 0;
        for i in 0..sprite.height() {
            draw_point += self.width * 4;

            for j in 0..sprite.width() {
                if sprite.pixels()[count] == 1 {
                    println!("draw point + j * 4: {}, j: {}", draw_point + j * 4, j);
                    pixels_screen[draw_point + (j * 4)] = 0xE2;
                    pixels_screen[draw_point + (j * 4) + 1] = 0x1B;
                    pixels_screen[draw_point + (j * 4) + 2] = 0x88;
                    pixels_screen[draw_point + (j * 4) + 3] = 0xff;
                }
                count += 1;
            }
        }
    }
}

/// Drawables can be blitted to the pixel buffer and animated.
pub trait Drawable {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &[u8];
}

/// A tiny position vector.
pub struct Point {
    pub x: usize,
    pub y: usize,
}
