// Draw provides basic drawing capabilities for blitting drawables to the chip8 display
// Display 64 x 32 pixels monochrome
pub struct Draw<'a> {
    width: usize,
    screen: &'a mut [u8],
    // sprites: Vec<u16>,
}
impl Draw<'_> {
    pub fn new(width: u32, screen: &mut [u8]) -> Draw {
        Draw {
            width: width as usize,
            screen,
        }
    }

    // bits raw hexadecimal values to the screen at the given destination
    pub fn blit_raw(&mut self, pixels: &[u8], dest: &Point, height: u8) -> bool {
        // calculate the base point: where to draw the sprite
        // multiply by 4 because there is one byte for the pixel and 3 bytes for the color
        let mut draw_point = (self.width * 4 * dest.y) + dest.x * 4;

        // if any pixels are turned off, set the flag register
        let mut set_flag_register = false;

        // TODO: create a final pixels array to store the data and then blit all in one shot
        // let mut final_pixels: Vec<u8> = Vec::new();

        // loop through the height
        for i in 0..height {
            // get the pixel
            let byte = &pixels[i as usize];
            for (index, byte_i) in (0..8).rev().enumerate() {
                let bit = (byte >> byte_i) & 1;
                let loc = draw_point + (index * 4);

                if bit == 1 && (loc < self.screen.len()) {
                    // if data already exists flip the bit to
                    if self.screen[loc] > 0x0
                        && self.screen[loc + 1] > 0x0
                        && self.screen[loc + 2] > 0x0
                        && self.screen[loc + 3] > 0x0
                    {
                        self.screen[loc] = 0x0;
                        self.screen[loc + 1] = 0x0;
                        self.screen[loc + 2] = 0x0;
                        self.screen[loc + 3] = 0x0;

                        set_flag_register = true;
                    } else {
                        self.screen[loc] = 0xE2;
                        self.screen[loc + 1] = 0x1B;
                        self.screen[loc + 2] = 0x88;
                        self.screen[loc + 3] = 0xFF;
                    }
                }
            }
            draw_point += self.width * 4;
        }

        set_flag_register
    }

    /* clears the screen */
    pub fn clear(&mut self) {
        for element in self.screen.iter_mut() {
            *element = 0;
        }
    }
}

/* A position vector */
#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/*  Drawables can be blitted to the pixel buffer and animated. */
pub trait Drawable {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &[u8];
}
