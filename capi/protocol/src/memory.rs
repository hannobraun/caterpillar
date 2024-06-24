use capi_process::Value;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Memory {
    #[serde(with = "serde_big_array::BigArray")]
    pub inner: [Value; 256],
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            inner: [Value(0); 256],
        }
    }
}
