use std::{collections::VecDeque, slice};

use crate::cp::runtime::data_stack::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnalyzerOutput {
    pub events: Vec<AnalyzerEvent>,
}

impl AnalyzerOutput {
    pub fn all_events_recursive(&self) -> AllEventsRecursive {
        let mut iters = VecDeque::new();
        iters.push_back(self.events.iter());

        AllEventsRecursive { iters }
    }
}

// This representation omits module, function, and test definitions, because the
// evaluator (which is the consumer of this representation) doesn't need them.
// This might be a problem.
//
// The idea is to have this be the canonical representation of the code. This
// representation will be stored in some kind of "system image", and the human-
// readable representation will be generated on the fly, whenever the developer
// looks at a function.
//
// Not having important parts of the language as part of the canonical
// representation seems like an obvious no-go at first, but it *could* actually
// work.
//
// A normal language doesn't need a representation of the file that the source
// code is defined in, because the concept of files exists "below" the actual
// language. It's something the editor needs to know about, not the language
// itself.
//
// In much the same way, the concept of a function could exist below the rest of
// the language in Caterpillar. It's the unit of organization, a thing that the
// system image and the editor need to understand, but not the language itself.
// The language itself needs to understand that functions exist and can be
// called, but it doesn't need to represent function definitions. The same
// thinking can be applied to modules and tests.
//
// However, handling it like this would create limitations. It would not be
// possible to have nested function definitions (because how would you represent
// the outer function then). Same goes for modules that have anything in them
// but function definitions (so no compile-time code that you can just write
// into a module).
//
// Maybe these limitations are fine, at first. Or maybe the solution is to
// broaden this representation beyond just what's needed by the evaluator. Maybe
// it makes most sense long-term, if there is a more general canonical
// representation, and the evaluator is just one of several consumers that only
// care about a subset.
//
// If there was a more general canonical representation, then this would
// probably not be it. Instead, this could be reframed as the set of events that
// the analyzer emits, and those events would be aggregated to build up the new
// canonical representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnalyzerEvent {
    Array { expressions: AnalyzerOutput },
    Binding { idents: Vec<String> },
    EvalBinding { name: String },
    EvalFunction { name: String },
    Module { name: String, body: AnalyzerOutput },
    Value(Value),
}

pub struct AllEventsRecursive<'r> {
    iters: VecDeque<slice::Iter<'r, AnalyzerEvent>>,
}

impl<'r> Iterator for AllEventsRecursive<'r> {
    type Item = &'r AnalyzerEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let next = loop {
            let iter = self.iters.front_mut()?;

            match iter.next() {
                Some(item) => break item,
                None => {
                    self.iters.pop_front();
                }
            }
        };

        let expressions = match next {
            AnalyzerEvent::Array { expressions } => Some(expressions),
            AnalyzerEvent::Module { body, .. } => Some(body),
            _ => None,
        };
        if let Some(iters) = expressions {
            self.iters.push_back(iters.events.iter());
        }

        Some(next)
    }
}
