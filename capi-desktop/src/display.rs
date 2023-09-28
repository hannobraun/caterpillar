pub struct Display {}

impl Display {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }

    pub fn set(&mut self, position: [i64; 2]) -> anyhow::Result<()> {
        tracing::info!("Should set pixel at {position:?}");
        Ok(())
    }
}
