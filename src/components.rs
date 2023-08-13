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

// // 16 8-bit (one byte) general-purpose variable registers numbered 0 through F hexadecimal, ie. 0 through 15 in decimal, called V0 through VF
// // VF is also used as a flag register; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag
pub struct GeneralRegisters {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    v_a: u8,
    v_b: u8,
    v_c: u8,
    v_d: u8,
    v_e: u8,
    v_f: u8, // flag register
    i: u16,  // generally used to store memory addresses
}
impl GeneralRegisters {
    pub fn new() -> GeneralRegisters {
        GeneralRegisters {
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            v_a: 0,
            v_b: 0,
            v_c: 0,
            v_d: 0,
            v_e: 0,
            v_f: 0,
            i: 0,
        }
    }
}
