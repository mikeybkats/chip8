pub struct Stack {
    sp: usize,
    stack: [u16; 512],
}
impl Stack {
    pub fn new() -> Stack {
        Stack {
            sp: 1,
            stack: [0; 512],
        }
    }

    pub fn push(&mut self, value: u16) {
        self.stack[self.sp + 1] = value;
        self.sp_increment();
    }

    pub fn pop(&mut self) -> u16 {
        let return_value = self.stack[self.sp];
        self.stack[self.sp] = 0;
        self.sp_decrement();
        return_value
    }

    pub fn sp_increment(&mut self) -> usize {
        self.sp += 1;
        self.sp
    }

    pub fn sp_decrement(&mut self) -> usize {
        if self.sp > 0 {
            self.sp -= 1;
        }
        self.sp
    }
}
