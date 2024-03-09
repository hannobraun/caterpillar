use core::num::Wrapping;

/// Access to the evaluator's data memory
pub struct Data {
    /// Points to the address where the *next* stack item will be pushed
    ///
    /// Need to be `Wrapping`, as that's what's going to happen, if the stack
    /// fully fills the available memory.
    stack_ptr: Wrapping<usize>,
}

impl Data {
    pub fn new(memory: &[u8]) -> Self {
        // Let's make `ptr` wrapping before doing any arithmetic. Otherwise, we
        // subtract with overflow, if `data` has zero length.
        let mut ptr = Wrapping(memory.len());
        ptr -= 1;

        Self { stack_ptr: ptr }
    }

    pub fn push(&mut self, value: u8, memory: &mut [u8]) {
        memory[self.stack_ptr.0] = value;
        self.stack_ptr -= 1;
    }

    pub fn pop(&mut self, memory: &mut [u8]) -> u8 {
        self.stack_ptr += 1;
        memory[self.stack_ptr.0]
    }

    pub fn store(
        &mut self,
        address: impl Into<usize>,
        value: u8,
        memory: &mut [u8],
    ) {
        memory[address.into()] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn fill_memory_completely() {
        let mut memory = [0; 1];
        let mut data = Data::new(&memory);

        data.push(0, &mut memory);
        // Should not panic. It will, in debug mode, unless wrapping is handled
        // correctly.
    }

    #[test]
    fn handle_zero_memory() {
        Data::new(&[]);
        // Should not panic. It will, unless wrapping behavior is handled
        // correctly when initializing the stack pointer.
    }
}
