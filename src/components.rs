use std::thread;
use std::time::Duration;

// // One 16-bit index register called “I” which is used to point at locations in memory
// pub struct IRegister {}
// impl IRegister {}

// // A stack for 16-bit addresses, which is used to call subroutines/functions and return from them
// pub struct Stack {}
// impl Stack {}

// // An 8-bit delay timer which is decremented at a rate of 60 Hz (60 times per second) until it reaches 0
// The delay timer is active whenever the delay timer register (DT) is non-zero. This timer does nothing more than subtract 1 from the value of DT at a rate of 60Hz. When DT reaches 0, it deactivates.
struct DelayTimer {
    register: bool,
    dt: u8,
}

impl DelayTimer {
    pub fn _new() -> DelayTimer {
        DelayTimer {
            register: false,
            dt: 60,
        }
    }

    pub fn _set_timer(&mut self, run: bool) {
        self.register = run;
        self._update();
    }

    pub fn _get_time(&self) -> u8 {
        self.dt
    }

    fn _update(&mut self) {
        if self.register == true {
            self._run_timer()
        }
    }

    // this will not work as implemented here.
    // the timer must run independent of the other parts of the application
    /**
    *
    *
       let mut dt = components::DelayTimer::new();
       dt.set_timer(true);

       loop {
           // infinite loop
           let t = dt.get_time();
           if t == 1 {
               dt.set_timer(false);
               break;
           }
       }
    */
    fn _run_timer(&mut self) {
        let delay = Duration::from_secs_f32(1.0 / 60.0);

        loop {
            let start_time = std::time::Instant::now();

            // Your loop code goes here
            self.dt -= 1;
            println!("delay timer: {}", self.dt);

            if self.dt == 0 || self.register == false {
                break;
            }

            let elapsed_time = start_time.elapsed();
            if elapsed_time < delay {
                thread::sleep(delay - elapsed_time);
            } else {
                println!("Loop took longer than expected!");
            }
        }
    }
}

// // An 8-bit sound timer which functions like the delay timer, but which also gives off a beeping sound as long as it’s not 0
// pub struct SoundTimer {}
// impl SoundTimer {}
