// Memory: CHIP-8 has direct access to up to 4 kilobytes of RAM
pub struct Memory {
    mem: [i8; 4096],
}
impl Memory {
    // the first 512 blocks of memory are empty, because the original chip8 used these to store the interpreter software
    pub fn new() -> Memory {
        Memory { mem: [0; 4096] }
    }

    pub fn length(&self) -> usize {
        self.mem.len()
    }
}

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

    pub fn loop_window(self) -> Result<(), pixels::Error> {
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
}
// A program counter, often called just “PC”, which points at the current instruction in memory
pub struct ProgramCounter {
    count: u8,
}
impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter { count: 0 }
    }

    pub fn clock(&mut self) -> u8 {
        self.count += 1;
        self.count
    }

    pub fn clear(&mut self) -> u8 {
        self.count = 0;
        self.count
    }

    pub fn get_count(&self) -> u8 {
        self.count
    }
}
#[cfg(test)]
mod program_counter_tests {
    // super brings the ProgramCounter into scope
    use super::*;

    #[test]
    fn can_create() {
        let pc = ProgramCounter::new();

        assert!(pc.get_count() == 0);
    }

    #[test]
    fn can_clock() {
        let mut pc = ProgramCounter::new();

        println!("the count is: {}", pc.count);
        assert!(pc.count == 0);
        pc.clock(); // 1
        pc.clock(); // 2
        assert!(pc.get_count() == 2);

        pc.clock(); // 3
        assert!(pc.get_count() == 3);
        assert!(pc.count == 3);
    }

    #[test]
    fn can_clear() {
        let mut pc = ProgramCounter::new();

        pc.clock(); // 1
        pc.clock(); // 2
        assert!(pc.get_count() == 2);

        pc.clear(); // 0
        assert!(pc.get_count() == 0);
    }
}

// // One 16-bit index register called “I” which is used to point at locations in memory
// pub struct IRegister {}
// impl IRegister {}

// // A stack for 16-bit addresses, which is used to call subroutines/functions and return from them
// pub struct Stack {}
// impl Stack {}

// // An 8-bit delay timer which is decremented at a rate of 60 Hz (60 times per second) until it reaches 0
// pub struct DelayTimer {}
// impl DelayTimer {}

// // An 8-bit sound timer which functions like the delay timer, but which also gives off a beeping sound as long as it’s not 0
// pub struct SoundTimer {}
// impl SoundTimer {}

// // 16 8-bit (one byte) general-purpose variable registers numbered 0 through F hexadecimal, ie. 0 through 15 in decimal, called V0 through VF
// // VF is also used as a flag register; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag
// pub struct GeneralRegisters {}
// impl GeneralRegisters {}
