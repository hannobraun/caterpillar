use capi_process::{Bytecode, Process, Value};

use crate::host::GameEngineHost;

pub struct GameEngine {
    pub arguments: Vec<Value>,
    pub bytecode: Option<Bytecode>,
    pub process: Process<GameEngineHost>,
}
