use super::syntax::SyntaxElement;

pub fn resolve(syntax: Vec<SyntaxElement>) -> Vec<Expression> {
    syntax
        .into_iter()
        .map(|syntax_element| match syntax_element {
            SyntaxElement::Value(value) => Expression::Value(value),
            SyntaxElement::Word { name } => Expression::Word { name },
        })
        .collect()
}

pub enum Expression {
    Value(usize),
    Word { name: &'static str },
}
