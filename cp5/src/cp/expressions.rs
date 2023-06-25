use std::{collections::VecDeque, slice};

use super::data_stack::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expressions {
    pub elements: Vec<Expression>,
}

impl Expressions {
    pub fn iter_recursive(&self) -> IterRecursive {
        let mut iters = VecDeque::new();
        iters.push_back(self.elements.iter());

        IterRecursive { iters }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Array { expressions: Expressions },
    Binding { idents: Vec<String> },
    EvalBinding { name: String },
    EvalFunction { name: String },
    Module { name: String, body: Expressions },
    Value(Value),
}

pub struct IterRecursive<'r> {
    iters: VecDeque<slice::Iter<'r, Expression>>,
}

impl<'r> Iterator for IterRecursive<'r> {
    type Item = &'r Expression;

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
            Expression::Array { expressions } => Some(expressions),
            Expression::Module { body, .. } => Some(body),
            _ => None,
        };
        if let Some(iters) = expressions {
            self.iters.push_back(iters.elements.iter());
        }

        Some(next)
    }
}
