use super::functions::Functions;

#[derive(Debug)]
pub struct Syntax<'r> {
    #[allow(dead_code)]
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

    pub fn v(&mut self, value: usize) -> &mut Self {
        self.elements.push(SyntaxElement::Value(value));
        self
    }

    pub fn w(&mut self, name: &'static str) -> &mut Self {
        self.elements.push(SyntaxElement::Word { name });
        self
    }
}

#[derive(Debug)]
pub enum SyntaxElement {
    Value(usize),
    Word { name: &'static str },
}
