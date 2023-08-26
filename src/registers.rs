use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

// // 16 8-bit (one byte) general-purpose variable registers numbered 0 through F hexadecimal, ie. 0 through 15 in decimal, called V0 through VF
// // VF is also used as a flag register; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GeneralRegisters {
    VZero,
    VOne,
    VTwo,
    VThree,
    VFour,
    VFive,
    VSix,
    VSeven,
    VEight,
    VNine,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

pub struct Registers {
    general_registers: HashMap<GeneralRegisters, u8>,
    i_register: u16,
    delay_timer: u8,
    delay_timer_instant: Instant,
    delay_timer_duration: Duration,
}
impl Registers {
    pub fn new() -> Registers {
        let mut registers: HashMap<GeneralRegisters, u8> = HashMap::new();

        registers.insert(GeneralRegisters::VZero, 0);
        registers.insert(GeneralRegisters::VOne, 0);
        registers.insert(GeneralRegisters::VTwo, 0);
        registers.insert(GeneralRegisters::VThree, 0);
        registers.insert(GeneralRegisters::VFour, 0);
        registers.insert(GeneralRegisters::VFive, 0);
        registers.insert(GeneralRegisters::VSix, 0);
        registers.insert(GeneralRegisters::VSeven, 0);
        registers.insert(GeneralRegisters::VEight, 0);
        registers.insert(GeneralRegisters::VNine, 0);
        registers.insert(GeneralRegisters::VA, 0);
        registers.insert(GeneralRegisters::VB, 0);
        registers.insert(GeneralRegisters::VC, 0);
        registers.insert(GeneralRegisters::VD, 0);
        registers.insert(GeneralRegisters::VE, 0);
        registers.insert(GeneralRegisters::VF, 0);

        Registers {
            general_registers: registers,
            i_register: 0,
            delay_timer: 60,
            delay_timer_instant: Instant::now(),
            delay_timer_duration: Duration::new(0, 0),
        }
    }

    fn match_register(&self, index: u16) -> Option<GeneralRegisters> {
        match index {
            0x0 => Some(GeneralRegisters::VZero),
            0x1 => Some(GeneralRegisters::VOne),
            0x2 => Some(GeneralRegisters::VTwo),
            0x3 => Some(GeneralRegisters::VThree),
            0x4 => Some(GeneralRegisters::VFour),
            0x5 => Some(GeneralRegisters::VFive),
            0x6 => Some(GeneralRegisters::VSix),
            0x7 => Some(GeneralRegisters::VSeven),
            0x8 => Some(GeneralRegisters::VEight),
            0x9 => Some(GeneralRegisters::VNine),
            0xA => Some(GeneralRegisters::VA),
            0xB => Some(GeneralRegisters::VB),
            0xC => Some(GeneralRegisters::VC),
            0xD => Some(GeneralRegisters::VD),
            0xE => Some(GeneralRegisters::VE),
            0xF => Some(GeneralRegisters::VF),
            _ => None,
        }
    }

    pub fn set_register(&mut self, index: u16, value: u8) -> Option<u8> {
        let register = self.match_register(index).unwrap();
        self.general_registers.insert(register, value)
    }

    pub fn get_register(&self, index: u16) -> Option<&u8> {
        let register = self.match_register(index).unwrap();
        self.general_registers.get(&register)
    }

    pub fn set_i_register(&mut self, value: u16) {
        self.i_register = value
    }

    pub fn get_i_register(&self) -> &u16 {
        &self.i_register
    }

    pub fn get_delay_timer(&mut self) -> &u8 {
        // 1. Keep track of the time the last value was written to the timer register. When reading from the register, calculate how long ago it was written to, and subtract an appropriate value from the result.

        if self.delay_timer != 0 {
            // get the elapsed time since the last time the delay rimer was set
            self.delay_timer_duration = self.delay_timer_instant.elapsed();
            let elapsed_time = self.delay_timer_duration.as_millis() as u8;

            // setting this to 12 because of timing issues.
            // TODO: consider working exclusively in MS to have more accurate timekeeping
            let num_millis_in_1_60 = 12 as u8;

            // if elapsed time is greater then a difference can be figured from the total elapsed time and subtracted from the delay timer
            if elapsed_time > num_millis_in_1_60 {
                // println!("elapsed time: {} ms", elapsed_time);
                // find the number of ticks to subtract from DT total
                let difference = elapsed_time / (num_millis_in_1_60);

                if difference >= 1 {
                    // check to make sure that the delay timer has enough value for the operation
                    if self.delay_timer >= difference {
                        // set the timer with the local method so that it can restart the timer Instant
                        self.set_delay_timer(self.delay_timer - difference);
                        // println!("difference: {} ms", difference);
                    } else {
                        self.set_delay_timer(self.delay_timer - 1);
                    }
                }
                self.delay_timer;
                // println!("dt: {} ticks", self.delay_timer);
            }
        }
        &self.delay_timer
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer_instant = Instant::now();
        self.delay_timer = value
    }
}
