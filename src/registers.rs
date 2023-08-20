use std::collections::HashMap;

// // 16 8-bit (one byte) general-purpose variable registers numbered 0 through F hexadecimal, ie. 0 through 15 in decimal, called V0 through VF
// // VF is also used as a flag register; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag
#[derive(Debug, PartialEq, Eq, Hash)]
enum GeneralRegisters {
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

    pub fn set_register(&mut self, register: GeneralRegisters, value: u8) {
        if value <= 0xF {
            self.registers.insert(register, value);
        } else {
            panic!("Invalid value for register");
        }
    }

    pub fn get_register(&self, index: u16) -> Option<&u8> {
        println!("get register: {}", index);
        match index {
            0x0 => self.registers.get(&GeneralRegisters::VZero),
            0x1 => self.registers.get(&GeneralRegisters::VOne),
            0x2 => self.registers.get(&GeneralRegisters::VTwo),
            0x3 => self.registers.get(&GeneralRegisters::VThree),
            0x4 => self.registers.get(&GeneralRegisters::VFour),
            0x5 => self.registers.get(&GeneralRegisters::VFive),
            0x6 => self.registers.get(&GeneralRegisters::VSix),
            0x7 => self.registers.get(&GeneralRegisters::VSeven),
            0x8 => self.registers.get(&GeneralRegisters::VEight),
            0x9 => self.registers.get(&GeneralRegisters::VNine),
            0xA => self.registers.get(&GeneralRegisters::VA),
            0xB => self.registers.get(&GeneralRegisters::VB),
            0xC => self.registers.get(&GeneralRegisters::VC),
            0xD => self.registers.get(&GeneralRegisters::VD),
            0xE => self.registers.get(&GeneralRegisters::VE),
            0xF => self.registers.get(&GeneralRegisters::VF),
            _ => None,
        }
    }
}
