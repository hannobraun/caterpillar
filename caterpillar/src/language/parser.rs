use super::tokenizer::Token;

pub struct Parser<'r> {
    tokens: &'r mut dyn Iterator<Item = Token>,
}

impl<'r> Parser<'r> {
    pub fn new(tokens: &'r mut dyn Iterator<Item = Token>) -> Self {
        Self { tokens }
    }
}

impl<'r> Iterator for Parser<'r> {
    type Item = SyntaxTree;

    fn next(&mut self) -> Option<Self::Item> {
        for token in &mut self.tokens {
            match token {
                Token::Fn { name } => {
                    return Some(SyntaxTree::Fn { name });
                }
                Token::ArrayOpen => {}
                Token::ArrayClose => {}
            }
        }

        None
    }
}

pub enum SyntaxTree {
    /// A function
    Fn { name: String },
}
