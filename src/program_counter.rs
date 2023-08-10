// A program counter, often called just “PC”, which points at the current instruction in memory
pub struct ProgramCounter {
    index: u16,
}
impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter { index: 0 }
    }

    pub fn increment(&mut self) -> u16 {
        self.index += 1;
        self.index
    }

    pub fn get_index(&self) -> usize {
        self.index as usize
    }
}
#[cfg(test)]
mod program_counter_tests {
    // super brings the ProgramCounter into scope
    use super::*;

    #[test]
    fn can_create() {
        let pc = ProgramCounter::new();

        assert!(pc.get_index() == 0);
    }

    #[test]
    fn can_increment() {
        let mut pc = ProgramCounter::new();

        println!("the count is: {}", pc.index);
        assert!(pc.index == 0);
        pc.increment(); // 1
        pc.increment(); // 2
        assert!(pc.get_index() == 2);

        pc.increment(); // 3
        assert!(pc.get_index() == 3);
        assert!(pc.index == 3);
    }

    #[test]
    fn can_clear() {
        let mut pc = ProgramCounter::new();

        pc.increment(); // 1
        pc.increment(); // 2
        assert!(pc.get_index() == 2);

        // pc.clear(); // 0
        // assert!(pc.get_index() == 0);
    }
}
