use super::tokenizer::Token;

pub struct Parser<'r> {
    pub tokens: &'r mut dyn Iterator<Item = Token>,
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
