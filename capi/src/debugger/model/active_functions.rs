use crate::process::Process;

use super::Function;

pub enum ActiveFunctions {
    Functions { functions: Vec<Function> },
}

impl ActiveFunctions {
    pub fn new(process: &Process) -> Self {
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
