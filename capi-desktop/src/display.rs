pub struct Display {}

impl Display {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }

    pub fn set(&mut self, position: [i64; 2]) {
        tracing::info!("Should set pixel at {position:?}");
    }
}

impl Default for Display {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
