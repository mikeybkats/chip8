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

use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
}
use winit::dpi::LogicalSize;

// Display: 64 x 32 pixels (or 128 x 64 for SUPER-CHIP) monochrome, ie. black or white
pub struct Display {
    width: u32,
    height: u32,
    event_loop: EventLoop<()>
}
impl Display {
    fn new()-> Display {
        Display { width: 64, height: 32, event_loop: EventLoop::new() }
    }

    fn build_window(self){
        // let event_loop = EventLoop::new();

        let window = {
            let size = LogicalSize::new(self.width as f64, self.height as f64);
            WindowBuilder::new()
                .with_title("Hello Pixels")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&self.event_loop)
                .unwrap()
        };
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
