use std::num::Wrapping;

/// A downward-growing stack
pub struct Data<'r> {
    /// Points to the address where the *next* item will be pushed
    ///
    /// Need to be `Wrapping`, as that's what's going to happen, if the stack
    /// fully fills the available memory.
    ptr: Wrapping<usize>,

    pub data: &'r mut [u8],
}

impl<'r> Data<'r> {
    pub fn new(data: &'r mut [u8]) -> Self {
        // Let's make `ptr` wrapping before doing any arithmetic. Otherwise, we
        // subtract with overflow, if `data` has zero length.
        let mut ptr = Wrapping(data.len());
        ptr -= 1;

        Self { ptr, data }
    }

    pub fn push(&mut self, value: u8) {
        self.data[self.ptr.0] = value;
        self.ptr -= 1;
    }

    pub fn pop(&mut self) -> u8 {
        self.ptr += 1;
        let value = self.data[self.ptr.0];
        value
    }

    pub fn store(&mut self, address: impl Into<usize>, value: u8) {
        self.data[address.into()] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn fill_memory_completely() {
        let mut data = [0; 1];
        let mut data = Data::new(&mut data);

        data.push(0);
        // Should not panic. It will, in debug mode, unless wrapping is handled
        // correctly.
    }

    #[test]
    fn handle_zero_memory() {
        let mut data = [];
        Data::new(&mut data);
        // Should not panic. It will, unless wrapping behavior is handled
        // correctly when initializing the stack pointer.
    }
}
