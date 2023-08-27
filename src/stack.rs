pub struct Stack {
    sp: usize,
    stack: [u16; 512],
}
impl Stack {
    pub fn new() -> Stack {
        Stack {
            sp: 0,
            stack: [0; 512],
        }
    }

    pub fn current(&self) -> u16 {
        self.stack[self.sp as usize]
    }

    pub fn push(&mut self, value: u16) {
        self.stack[self.sp] = value;
        self.sp_increment();
    }

    // pub fn pop(&mut self) -> u16 {
    //     let return_value = self.stack[self.sp];
    //     self.decrement();
    //     return_value
    // }

    pub fn sp_increment(&mut self) -> usize {
        self.sp += 1;
        self.sp
    }

    // pub fn decrement(&mut self) -> usize {
    //     self.sp -= 1;
    //     self.sp
    // }

    pub fn jump_to_address(&mut self, value: usize) {
        self.sp = value;
    }

    // pub fn get_sp(&self) -> usize {
    //     self.sp
    // }
}
