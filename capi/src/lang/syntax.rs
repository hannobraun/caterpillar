#[derive(Debug)]
pub struct Syntax<'r> {
    elements: &'r mut Vec<SyntaxElement>,
}

impl<'r> Syntax<'r> {
    pub fn new(elements: &'r mut Vec<SyntaxElement>) -> Self {
        Self { elements }
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
