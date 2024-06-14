use crate::process::Process;

use super::Function;

#[derive(Clone)]
pub enum ActiveFunctions {
    Functions { functions: Vec<Function> },
    Message { message: &'static str },
}

impl ActiveFunctions {
    pub fn new(process: Option<&Process>) -> Self {
        let Some(process) = process else {
            return Self::Message {
                message: "No process available.",
            };
        };

        if process.can_step() {
            return Self::Message {
                message: "Process is running.",
            };
        }

        let functions = process
            .evaluator
            .stack()
            .iter()
            .filter_map(|runtime_location| {
                let syntax_location =
                    process.source_map.runtime_to_syntax(&runtime_location);
                let function = process
                    .functions
                    .get_from_location(syntax_location)
                    .cloned()?;

                Some(Function::new(function, process))
            })
            .collect();

        Self::Functions { functions }
    }
}
