use std::fmt;

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Value(pub i32);

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = self.0.to_le_bytes();

        write!(f, "0x")?;
        for b in bytes.into_iter().rev() {
            write!(f, "{b:02x}")?;
        }

        Ok(())
    }
}
