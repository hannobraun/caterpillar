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
        let mut arrays: Vec<Vec<_>> = Vec::new();

        for token in &mut self.tokens {
            match token {
                Token::Fn { name } => {
                    let syntax_tree = SyntaxTree::Fn { name };
                    if let Some(array) = arrays.last_mut() {
                        array.push(syntax_tree);
                    } else {
                        return Some(syntax_tree);
                    }
                }
                Token::ArrayOpen => {
                    arrays.push(Vec::new());
                }
                Token::ArrayClose => {
                    if let Some(syntax_tree) = arrays.pop() {
                        return Some(SyntaxTree::Array { syntax_tree });
                    }

                    // If there's no array open, nothing will happen. We can
                    // make this stricter, later on.
                }
            }
        }

        None
    }
}

pub enum SyntaxTree {
    /// A function
    Fn { name: String },

    /// An array
    Array { syntax_tree: Vec<SyntaxTree> },
}
