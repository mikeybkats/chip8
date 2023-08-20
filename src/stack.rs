pub struct Stack {
    sp: usize,
    stack: [u8; 512],
}
impl Stack {
    pub fn new() -> Stack {
        Stack {
            sp: 0,
            stack: [0; 512],
        }
    }

    pub fn current(&self) -> u8 {
        self.stack[self.sp as usize]
    }

    pub fn push(mut self, value: u8) {
        self.stack[self.sp] = value;
        self.increment();
    }

    pub fn pop(mut self) -> u8 {
        let return_value = self.stack[self.sp];
        self.decrement();
        return_value
    }

    pub fn increment(&mut self) -> u8 {
        self.sp += 1;
        self.sp as u8
    }

    pub fn decrement(&mut self) -> u8 {
        self.sp -= 1;
        self.sp as u8
    }

    pub fn jump_to_address(&mut self, value: usize) {
        self.sp = value;
    }

    pub fn get_sp(&self) -> u8 {
        self.sp as u8
    }
}
