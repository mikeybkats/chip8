// A program counter, often called just “PC”, which points at the current instruction in memory
pub struct ProgramCounter {
    pc: u16,
    // rom: [u8; 3584],
}
impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter { pc: 0 }
    }

    pub fn set_counter(&mut self, new_counter: u16) {
        self.pc = new_counter
    }

    pub fn increment_by(&mut self, value: u16) -> u16 {
        self.pc += value;
        self.pc
    }

    pub fn increment(&mut self) -> u16 {
        self.pc += 1;
        self.pc
    }

    pub fn decrement(&mut self) -> u16 {
        self.pc -= 1;
        self.pc
    }

    pub fn jump(&mut self, location: u16) {
        self.pc = location;
    }

    // gets the location of the program counter
    pub fn get_pc(&self) -> usize {
        self.pc as usize
    }
}
#[cfg(test)]
mod program_counter_tests {
    // super brings the ProgramCounter into scope
    use super::*;

    #[test]
    fn can_create() {
        let pc = ProgramCounter::new();

        assert!(pc.get_pc() == 0);
    }

    #[test]
    fn can_increment() {
        let mut pc = ProgramCounter::new();

        assert!(pc.pc == 0);
        pc.increment(); // 1
        pc.increment(); // 2
        assert!(pc.get_pc() == 2);

        pc.increment(); // 3
        assert!(pc.get_pc() == 3);
        assert!(pc.pc == 3);
    }

    #[test]
    fn can_clear() {
        let mut pc = ProgramCounter::new();

        pc.increment(); // 1
        pc.increment(); // 2
        assert!(pc.get_pc() == 2);
    }
}
