use super::functions::Functions;

#[derive(Debug)]
pub struct Syntax<'r> {
    functions: &'r Functions,
    elements: &'r mut Vec<SyntaxElement>,
}

impl<'r> Syntax<'r> {
    pub fn new(
        functions: &'r Functions,
        elements: &'r mut Vec<SyntaxElement>,
    ) -> Self {
        Self {
            functions,
            elements,
        }
    }

    pub fn b(&mut self, name: &'static str) -> &mut Self {
        self.elements.push(SyntaxElement::CallBuiltin { name });
        self
    }

    pub fn f(&mut self, name: &'static str) -> &mut Self {
        let address = self.functions.resolve(name);
        self.elements.push(SyntaxElement::CallFunction { address });
        self
    }

    pub fn v(&mut self, value: usize) -> &mut Self {
        self.elements.push(SyntaxElement::PushValue(value));
        self
    }
}

#[derive(Debug)]
pub enum SyntaxElement {
    CallBuiltin { name: &'static str },
    CallFunction { address: usize },
    PushValue(usize),
}
