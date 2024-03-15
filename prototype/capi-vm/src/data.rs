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

    pub fn push(&mut self, value: u32, data: &mut [u8]) {
        let bytes = value.to_le_bytes();

        for b in bytes.into_iter().rev() {
            data[self.stack_ptr.0] = b;
            self.stack_ptr -= 1;
        }
    }

    pub fn pop(&mut self, data: &mut [u8]) -> u32 {
        let mut bytes = [0; 4];

        for b in bytes.iter_mut() {
            self.stack_ptr += 1;
            *b = data[self.stack_ptr.0];
        }

        u32::from_le_bytes(bytes)
    }

    pub fn store(
        &mut self,
        address: u32,
        value: u32,
        data: &mut [u8],
    ) -> Result<(), StoreError> {
        let mut address: usize = address
            .try_into()
            .expect("Couldn't convert address to usize");
        let bytes = value.to_le_bytes();

        if address + bytes.len() >= data.len() {
            return Err(StoreError {
                value,
                address,
                data_len: data.len(),
            });
        }

        for b in bytes {
            data[address] = b;
            address += 1;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct StoreError {
    pub value: u32,
    pub address: usize,
    pub data_len: usize,
}

#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn fill_memory_completely() {
        let mut memory = [0; 4];
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
