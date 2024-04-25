#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct DataStack {
    values: Vec<usize>,
    saved: Vec<usize>,
}

impl DataStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clone(&mut self) -> usize {
        self.values.last().copied().unwrap()
    }

    pub fn push(&mut self, value: usize) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Result<usize, PopFromEmptyStack> {
        self.values.pop().ok_or(PopFromEmptyStack)
    }

    pub fn num_values(&self) -> usize {
        self.values.len()
    }

    pub fn save(&mut self, num: usize) {
        for _ in 0..num {
            let value = self.pop().unwrap();
            self.saved.push(value);
        }
    }

    pub fn restore(&mut self) {
        while let Some(x) = self.saved.pop() {
            self.push(x);
        }
    }
}

#[derive(Debug)]
pub struct PopFromEmptyStack;
