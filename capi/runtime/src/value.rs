use std::fmt;

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Value(pub [u8; 4]);

impl Value {
    pub fn to_i8(&self) -> Result<i8, IntegerOverflow> {
        let [v, 0, 0, 0] = self.0 else {
            return Err(IntegerOverflow);
        };
        Ok(i8::from_le_bytes([v]))
    }

    pub fn to_i32(&self) -> i32 {
        i32::from_le_bytes(self.0)
    }

    pub fn to_u8(&self) -> Result<u8, IntegerOverflow> {
        let [v, 0, 0, 0] = self.0 else {
            return Err(IntegerOverflow);
        };
        Ok(u8::from_le_bytes([v]))
    }

    pub fn to_u32(&self) -> u32 {
        u32::from_le_bytes(self.0)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        let [v] = value.to_le_bytes();
        Value([v, 0, 0, 0])
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self(value.to_le_bytes())
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        let value: u32 = value.into();
        value.into()
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
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

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct IntegerOverflow;
