use std::fmt;

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Value(pub [u8; 4]);

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self(value.to_le_bytes())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x")?;
        for b in self.0.into_iter().rev() {
            write!(f, "{b:02x}")?;
        }

        Ok(())
    }
}
