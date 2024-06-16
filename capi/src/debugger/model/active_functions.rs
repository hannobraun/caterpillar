use crate::{process::Process, source_map::SourceMap, syntax};

use super::Function;

#[derive(Clone)]
pub enum ActiveFunctions {
    Functions { functions: Vec<Function> },
    Message { message: &'static str },
}

impl ActiveFunctions {
    pub fn new(
        source_code: Option<(&syntax::Functions, &SourceMap)>,
        process: Option<&Process>,
    ) -> Self {
        let Some((functions, source_map)) = source_code else {
            return Self::Message {
                message: "No connection to Caterpillar process.",
            };
        };
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
                    source_map.runtime_to_syntax(&runtime_location);
                let function =
                    functions.get_from_location(syntax_location).cloned()?;

                Some(Function::new(function, source_map, process))
            })
            .collect();

        Self::Functions { functions }
    }
}
