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

struct Registers {
    registers: HashMap<GeneralRegisters, u16>,
}
impl Registers {
    pub fn new() -> Registers {
        let mut registers: HashMap<GeneralRegisters, u16> = HashMap::new();

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

        Registers { registers }
    }

    fn set_register(&mut self, register: GeneralRegisters, value: u16) {
        if value <= 0xF {
            self.registers.insert(register, value);
        } else {
            panic!("Invalid value for register");
        }
    }

    fn get_register(&self, register: GeneralRegisters) -> Option<&u16> {
        self.registers.get(&register)
    }
}
