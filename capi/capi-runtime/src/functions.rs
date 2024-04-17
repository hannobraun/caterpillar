use std::collections::BTreeSet;

use crate::{syntax::Expression, DebugEvent, LineLocation};

use super::syntax::Syntax;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Functions {
    pub names: BTreeSet<String>,
    pub inner: Vec<Function>,
}

impl Functions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: &str, f: impl FnOnce(&mut Syntax)) {
        if self.names.contains(name) {
            panic!("Can't re-define existing function `{name}`.");
        }

        let mut syntax = Vec::new();
        f(&mut Syntax::new(name.to_string(), &mut syntax));

        self.names.insert(name.to_string());
        self.inner.push(Function {
            name: name.to_string(),
            syntax,
        });
    }

    pub fn apply_debug_event(&mut self, event: DebugEvent) {
        match event {
            DebugEvent::ToggleBreakpoint {
                location: LineLocation { function, line },
            } => {
                let line: usize = line.try_into().unwrap();

                let function =
                    self.inner.iter_mut().find(|f| f.name == function).unwrap();
                let syntax_element = function.syntax.get_mut(line).unwrap();

                syntax_element.breakpoint = !syntax_element.breakpoint;
            }
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub syntax: Vec<Expression>,
}
