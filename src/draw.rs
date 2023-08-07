// Display: 64 x 32 pixels (or 128 x 64 for SUPER-CHIP) monochrome, ie. black or white
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

    pub fn draw_pixel(&mut self, x: usize, y: usize) {
        let base_point = (y * (self.width as usize)) + x as usize;

        for (i, pixel) in self.screen.chunks_exact_mut(4).enumerate() {
            if i == base_point {
                println!("Copying to slice");
                pixel[0..4].copy_from_slice(&[0xE2, 0x1B, 0x88, 0xff]);
            }
        }
    }

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
