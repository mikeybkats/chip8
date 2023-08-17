// A program counter, often called just “PC”, which points at the current instruction in memory
pub struct ProgramCounter<'a> {
    pc: usize,
    rom: &'a mut [u8],
}
impl<'a> ProgramCounter<'a> {
    pub fn new(rom_memory: &'a mut [u8]) -> ProgramCounter {
        ProgramCounter {
            pc: 0,
            rom: rom_memory,
        }
    }

    pub fn increment(&mut self) -> usize {
        self.pc += 1;
        self.pc
    }

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
        let mut memory = [0 as u8; 1000];
        let pc = ProgramCounter::new(&mut memory);

        assert!(pc.get_pc() == 0);
    }

    #[test]
    fn can_increment() {
        let mut memory = [0 as u8; 1000];
        let mut pc = ProgramCounter::new(&mut memory);

        println!("the count is: {}", pc.pc);
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
        let mut memory = [0 as u8; 1000];
        let mut pc = ProgramCounter::new(&mut memory);

        pc.increment(); // 1
        pc.increment(); // 2
        assert!(pc.get_pc() == 2);

        // pc.clear(); // 0
        // assert!(pc.get_pc() == 0);
    }
}
