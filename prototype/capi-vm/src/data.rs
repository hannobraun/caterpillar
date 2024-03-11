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
    pub fn new(data: &[u8]) -> Self {
        // Let's make `ptr` wrapping before doing any arithmetic. Otherwise, we
        // subtract with overflow, if `data` has zero length.
        let mut ptr = Wrapping(data.len());
        ptr -= 1;

        Self { stack_ptr: ptr }
    }

    pub fn push<V>(&mut self, value: V, memory: &mut [u8])
    where
        V: IntoIterator<Item = u8>,
        V::IntoIter: DoubleEndedIterator,
    {
        for b in value.into_iter().rev() {
            memory[self.stack_ptr.0] = b;
            self.stack_ptr -= 1;
        }
    }

    pub fn pop(&mut self, value: &mut [u8], memory: &mut [u8]) {
        for b in value.iter_mut() {
            self.stack_ptr += 1;
            *b = memory[self.stack_ptr.0];
        }
    }

    pub fn store(
        &mut self,
        address: u32,
        value: impl IntoIterator<Item = u8>,
        memory: &mut [u8],
    ) {
        let mut address: usize = address
            .try_into()
            .expect("Couldn't convert address to usize");

        for b in value {
            memory[address] = b;
            address += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn fill_memory_completely() {
        let mut memory = [0; 1];
        let mut data = Data::new(&memory);

        data.push([0], &mut memory);
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
