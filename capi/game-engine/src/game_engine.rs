use std::collections::VecDeque;

use capi_process::{Bytecode, Process, Value};

use crate::{
    host::{GameEngineHost, TILES_PER_AXIS},
    input::Input,
    memory::Memory,
};

pub struct GameEngine {
    pub bytecode: Option<Bytecode>,
    pub process: Process<GameEngineHost>,
    pub arguments: [Value; 2],
    pub memory: Memory,
    pub input: Input,
    pub random: VecDeque<i32>,
}

impl GameEngine {
    pub fn new() -> Self {
        let memory = Memory::default();
        let input = Input::default();

        Self {
            bytecode: None,
            process: Process::default(),
            arguments: [Value::from(TILES_PER_AXIS as i32); 2],
            memory,
            input,
            random: VecDeque::new(),
        }
    }

    pub fn on_new_bytecode(&mut self, bytecode: Bytecode) {
        self.bytecode = Some(bytecode);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.memory = Memory::default();
        self.process.reset(self.arguments);
    }
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::new()
    }
}
