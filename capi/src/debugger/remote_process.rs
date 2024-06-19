use crate::{
    breakpoints,
    process::{self, Process},
    source_map::SourceMap,
    state::Memory,
    syntax,
    updates::Update,
};

use super::model::{ActiveFunctions, Debugger};

pub struct RemoteProcess {
    pub breakpoints: breakpoints::State,
    pub process2: process::State,
    pub source_code: Option<(syntax::Functions, SourceMap)>,
    pub process: Option<Process>,
    pub memory: Option<Memory>,
}

impl RemoteProcess {
    pub fn new() -> Self {
        Self {
            breakpoints: breakpoints::State::default(),
            process2: process::State::default(),
            source_code: None,
            process: None,
            memory: None,
        }
    }

    pub fn on_update(&mut self, update: Update) {
        match update {
            Update::Breakpoints { event } => {
                self.breakpoints.evolve(event);
            }
            Update::Memory { memory } => {
                self.memory = Some(memory);
            }
            Update::Process(process) => {
                self.process = Some(process);
            }
            Update::Process2 { event } => {
                self.process2.evolve(event);
            }
            Update::SourceCode {
                functions,
                source_map,
            } => {
                self.source_code = Some((functions, source_map));
            }
        }
    }

    pub fn to_debugger(&self) -> Debugger {
        let active_functions = ActiveFunctions::new(
            self.source_code
                .as_ref()
                .map(|(functions, source_map)| (functions, source_map)),
            &self.breakpoints,
            &self.process2,
            self.process.as_ref(),
        );
        let operands = self
            .process
            .as_ref()
            .map(|process| process.operands().unwrap().clone());
        let memory = self.memory.clone();

        Debugger {
            active_functions,
            operands,
            memory,
        }
    }
}
