use std::thread;
use std::time::{Duration, Instant};

const INSTRUCTIONS_PER_SECOND: u32 = 700;

type FnPointer = fn() -> bool;

pub fn clock_emulator() {
    let delay = Duration::from_secs(1) / INSTRUCTIONS_PER_SECOND;

    loop {
        let start_time = Instant::now();

        // let mut end = false;
        for _ in 0..INSTRUCTIONS_PER_SECOND {
            // Fetch and process the instruction
            // TODO: execute should return a bool when program is ready to end
        }

        // if end == true {
        //     break;
        // }

        let elapsed_time = start_time.elapsed();
        if elapsed_time < delay {
            thread::sleep(delay - elapsed_time);
        } else {
            println!("Took longer than expected to process instructions!");
        }

        if elapsed_time > Duration::new(10, 0) {
            break;
        }
    }
}
