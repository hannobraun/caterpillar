use std::{iter, num::Wrapping};

/// Access to the evaluator's data memory
pub struct Data {
    /// Points to the address where the *next* stack item will be pushed
    ///
    /// Need to be `Wrapping`, as that's what's going to happen, if the stack
    /// fully fills the available memory.
    stack_ptr: Wrapping<usize>,

    data: Vec<u8>,
}

impl Data {
    pub fn new(size: usize) -> Self {
        // Let's make `ptr` wrapping before doing any arithmetic. Otherwise, we
        // subtract with overflow, if `data` has zero length.
        let mut ptr = Wrapping(size);
        ptr -= 1;

        let data = iter::repeat(0).take(size).collect();

        Self {
            stack_ptr: ptr,
            data,
        }
    }

    pub fn read(&self) -> &[u8] {
        &self.data
    }

    pub fn push(&mut self, value: u8) {
        self.data[self.stack_ptr.0] = value;
        self.stack_ptr -= 1;
    }

    pub fn pop(&mut self) -> u8 {
        self.stack_ptr += 1;
        let value = self.data[self.stack_ptr.0];
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
        let mut data = Data::new(1);

        data.push(0);
        // Should not panic. It will, in debug mode, unless wrapping is handled
        // correctly.
    }

    #[test]
    fn handle_zero_memory() {
        Data::new(0);
        // Should not panic. It will, unless wrapping behavior is handled
        // correctly when initializing the stack pointer.
    }
}
