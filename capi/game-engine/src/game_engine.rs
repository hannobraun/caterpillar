use capi_process::{Bytecode, Value};

pub struct GameEngine {
    pub arguments: Vec<Value>,
    pub bytecode: Option<Bytecode>,
}
