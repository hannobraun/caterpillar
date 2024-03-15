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

    pub fn read_value<'b>(
        &mut self,
        buffer: &'b mut [u8],
        code: &[u8],
    ) -> impl Iterator<Item = u8> + DoubleEndedIterator + 'b {
        for b in buffer.iter_mut() {
            *b = code[self.ptr];
            self.ptr += 1;
        }

        buffer.iter().copied()
    }

    pub fn jump_relative(&mut self, offset: i8) {
        let ptr: isize = self
            .ptr
            .try_into()
            .expect("Couldn't convert code pointer to `isize`");
        let offset: isize = offset.into();

        let ptr = ptr
            .checked_add(offset)
            .expect("Relative jump caused overflow");

        let ptr: usize = ptr
            .try_into()
            .expect("Relative jump caused negative overflow");

        self.ptr = ptr;
    }
}
