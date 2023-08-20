use std::collections::HashMap;

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
    registers: HashMap<GeneralRegisters, u8>,
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

        Registers { registers }
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
        self.registers.insert(register, value)
    }

    pub fn get_register(&self, index: u16) -> Option<&u8> {
        let register = self.match_register(index).unwrap();
        self.registers.get(&register)
    }
}
