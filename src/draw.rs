// Draw provides basic drawing capabilities for blitting drawables to the chip8 display
// Display 64 x 32 pixels monochrome
pub struct Draw<'a> {
    width: usize,
    height: usize,
    screen: &'a mut [u8],
}
impl Draw<'_> {
    pub fn new(width: u32, height: u32, screen: &mut [u8]) -> Draw {
        Draw {
            width: width as usize,
            height: height as usize,
            screen,
        }
    }

    /* Draws pixel to x, y coordinates  */
    pub fn draw_pixel(&mut self, dest: &Point) {
        assert!(dest.x <= self.width);
        assert!(dest.y <= self.height);

        let base_point = (dest.y * (self.width as usize)) + dest.x as usize;

        for (i, pixel) in self.screen.chunks_exact_mut(4).enumerate() {
            if i == base_point {
                pixel[0..4].copy_from_slice(&[0xE2, 0x1B, 0x88, 0xff]);
            }
        }
    }

    /* blits the sprit to the viewport */
    // blit is shorthand for bit block transfer
    // it refers to the operation of copying a block of data to a block of pixels in memory
    pub fn blit_drawable<'a, E>(&mut self, dest: &Point, sprite: &E)
    where
        E: Drawable,
    {
        assert!(dest.x + sprite.width() <= self.width);
        assert!(dest.y + sprite.height() <= self.height);

        // calculate the base point: where to draw the sprite
        let mut draw_point = (self.width * 4 * dest.y) + dest.x * 4;

        let mut count = 0;
        for _i in 0..sprite.height() {
            for j in 0..sprite.width() {
                let loc = draw_point + (j * 4);
                if sprite.pixels()[count] == 1 {
                    self.screen[loc] = 0xE2;
                    self.screen[loc + 1] = 0x1B;
                    self.screen[loc + 2] = 0x88;
                    self.screen[loc + 3] = 0xff;
                }
                count += 1;
            }
            draw_point += self.width * 4;
        }
    }

    /* clears the screen */
    pub fn clear(&mut self) {
        for element in self.screen.iter_mut() {
            *element = 0;
        }
    }
}

/*  Drawables can be blitted to the pixel buffer and animated. */
pub trait Drawable {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &[u8];
}

/* A position vector */
pub struct Point {
    pub x: usize,
    pub y: usize,
}
