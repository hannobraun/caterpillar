#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Command {
    Continue,
    Reset,
    Step,
    Stop,
}
