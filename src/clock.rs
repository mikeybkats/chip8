use std::thread;
use std::time::{Duration, Instant};

const INSTRUCTIONS_PER_SECOND: u32 = 700;

type FnBox = Box<dyn Fn() -> bool>;
type FnPointer = fn() -> bool;

pub fn clock_emulator(instructions: Vec<FnPointer>) {
    let delay = Duration::from_secs(1) / INSTRUCTIONS_PER_SECOND;

    let mut instructions_iter = instructions.into_iter();

    loop {
        let start_time = Instant::now();

        let mut end = false;
        for _ in 0..INSTRUCTIONS_PER_SECOND {
            if let Some(instruction) = instructions_iter.next() {
                // Fetch and process the instruction
                // Decode
                // Execute
                end = instruction();
            }
        }

        if end == true {
            break;
        }

        let elapsed_time = start_time.elapsed();
        if elapsed_time < delay {
            thread::sleep(delay - elapsed_time);
        } else {
            println!("Took longer than expected to process instructions!");
        }
    }
}
