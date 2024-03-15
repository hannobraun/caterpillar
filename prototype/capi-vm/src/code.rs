pub struct Code {
    pub ptr: usize,
}

impl Code {
    pub fn new() -> Self {
        Self { ptr: 0 }
    }

    pub fn read_instruction(&mut self, code: &[u8]) -> Option<u8> {
        let instruction = code.get(self.ptr).copied();
        self.ptr += 1;
        instruction
    }
}
