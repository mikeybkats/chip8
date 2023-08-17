pub struct Stack<'a> {
    sp: usize,
    stack: &'a mut [u8],
}
impl<'a> Stack<'a> {
    pub fn new(stack_memory: &'a mut [u8]) -> Stack<'a> {
        Stack {
            sp: 0,
            stack: stack_memory,
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

    pub fn get_sp(&self) -> u8 {
        self.sp as u8
    }
}
