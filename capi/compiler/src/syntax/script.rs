use super::Function;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Script {
    pub functions: Vec<Function>,
}
