// Draw provides basic drawing capabilities for blitting drawables to the chip8 display
// Display 64 x 32 pixels monochrome
pub struct Draw<'a> {
    width: usize,
    screen: &'a mut [u8],
}
impl Draw<'_> {
    pub fn new(width: u32, screen: &mut [u8]) -> Draw {
        Draw {
            width: width as usize,
            screen,
        }
    }

    pub fn _binary_to_sprite(bin: &[u8]) -> Vec<u8> {
        let mut pixels: Vec<u8> = Vec::new();
        for row in bin.iter() {
            // {:b} is binary format
            // {:08b} formats the number as 8-bit binary with leading zeros
            let binary_string = format!("{:08b}", row);

            for bit in binary_string.chars() {
                if let Some(bit_value) = bit.to_digit(2) {
                    pixels.push(bit_value as u8);
                }
            }
        }
        pixels
    }

    // bits raw hexadecimal values to the screen at the given destination
    pub fn blit_raw(&mut self, pixels: &[u8], dest: &Point, height: u8) {
        // calculate the base point: where to draw the sprite
        // multiply by 4 because there is one byte for the pixel and 3 bytes for the color
        let mut draw_point = (self.width * 4 * dest.y) + dest.x * 4;

        // TODO: create a final pixels array to store the data and then blit all in one shot
        // let mut final_pixels: Vec<u8> = Vec::new();

        // loop through the height
        for i in 0..height {
            // get the pixel
            let byte = &pixels[i as usize];
            for (index, byte_i) in (0..8).rev().enumerate() {
                let bit = (byte >> byte_i) & 1;
                // print!("{}", bit);
                let loc = draw_point + (index * 4);
                // println!("loc: {}, byte_i: {}", loc, byte_i);

                if bit == 1 {
                    self.screen[loc] = 0xE2;
                    self.screen[loc + 1] = 0x1B;
                    self.screen[loc + 2] = 0x88;
                    self.screen[loc + 3] = 0xFF;
                } else {
                    // TODO: enable this block of code when ready
                    // if the pixel location contains data already then set it to black
                    if self.screen[loc] > 0
                        || self.screen[loc + 1] > 0
                        || self.screen[loc + 2] > 0
                        || self.screen[loc + 3] > 0
                    {
                        self.screen[loc] = 0x0;
                        self.screen[loc + 1] = 0x0;
                        self.screen[loc + 2] = 0x0;
                        self.screen[loc + 3] = 0x0;
                    }
                }
            }
            draw_point += self.width * 4;
        }
    }

    /* blits the sprit to the viewport */
    // blit is shorthand for bit block transfer
    // it refers to the operation of copying a block of data to a block of pixels in memory
    pub fn _blit_drawable<'a, E>(&mut self, dest: &Point, sprite: &E)
    where
        E: Drawable,
    {
        assert!(dest.x + sprite.width() <= self.width);
        // assert!(dest.y + sprite.height() <= self.height);

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
                    self.screen[loc + 3] = 0xFF;
                } else {
                    // TODO: enable this block of code when ready
                    // if the pixel location contains data already then set it to black
                    if self.screen[loc] > 0
                        || self.screen[loc + 1] > 0
                        || self.screen[loc + 2] > 0
                        || self.screen[loc + 3] > 0
                    {
                        self.screen[loc] = 0x0;
                        self.screen[loc + 1] = 0x0;
                        self.screen[loc + 2] = 0x0;
                        self.screen[loc + 3] = 0x0;
                    }
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
