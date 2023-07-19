use std::fmt;

use super::{pipeline::ir::analyzer_output::AnalyzerOutput, AnalyzerEvent};

pub struct Formatter<'r>(pub &'r AnalyzerOutput);

impl fmt::Display for Formatter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, event) in self.0.events.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }

            match event {
                AnalyzerEvent::Array { expressions } => {
                    write!(f, "{}", Self(expressions))?;
                }
                AnalyzerEvent::Binding { idents } => {
                    write!(f, "=> ")?;

                    for ident in idents {
                        write!(f, "{} ", ident)?;
                    }

                    write!(f, ".")?;
                }
                AnalyzerEvent::EvalBinding { name } => {
                    write!(f, "{name}")?;
                }
                AnalyzerEvent::EvalFunction { name } => {
                    write!(f, "{name}")?;
                }
                AnalyzerEvent::Module { name, body } => {
                    write!(f, "mod {name} {{ {} }}", Self(body))?;
                }
                AnalyzerEvent::Value(value) => {
                    write!(f, "{value}")?;
                }
            }
        }

        Ok(())
    }
}
