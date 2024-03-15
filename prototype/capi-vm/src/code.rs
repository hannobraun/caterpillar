#[derive(Debug)]
pub struct Code {
    ptr: usize,
}

impl Code {
    pub fn new() -> Self {
        Self { ptr: 0 }
    }

    pub fn reset(&mut self) {
        self.ptr = 0;
    }

    pub fn read_instruction(&mut self, code: &[u8]) -> Option<u8> {
        let instruction = code.get(self.ptr).copied();
        self.ptr += 1;
        instruction
    }

    pub fn read_value(&mut self, code: &[u8]) -> u32 {
        let mut buffer = [0; 4];

        for b in &mut buffer {
            *b = code[self.ptr];
            self.ptr += 1;
        }

        u32::from_le_bytes(buffer)
    }
}
