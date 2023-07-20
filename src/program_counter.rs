// A program counter, often called just “PC”, which points at the current instruction in memory
pub struct ProgramCounter {
    count: u8,
}
impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter { count: 0 }
    }

    pub fn clock(&mut self) -> u8 {
        self.count += 1;
        self.count
    }

    pub fn clear(&mut self) -> u8 {
        self.count = 0;
        self.count
    }

    pub fn get_count(&self) -> u8 {
        self.count
    }
}
#[cfg(test)]
mod program_counter_tests {
    // super brings the ProgramCounter into scope
    use super::*;

    #[test]
    fn can_create() {
        let pc = ProgramCounter::new();

        assert!(pc.get_count() == 0);
    }

    #[test]
    fn can_clock() {
        let mut pc = ProgramCounter::new();

        println!("the count is: {}", pc.count);
        assert!(pc.count == 0);
        pc.clock(); // 1
        pc.clock(); // 2
        assert!(pc.get_count() == 2);

        pc.clock(); // 3
        assert!(pc.get_count() == 3);
        assert!(pc.count == 3);
    }

    #[test]
    fn can_clear() {
        let mut pc = ProgramCounter::new();

        pc.clock(); // 1
        pc.clock(); // 2
        assert!(pc.get_count() == 2);

        pc.clear(); // 0
        assert!(pc.get_count() == 0);
    }
}
