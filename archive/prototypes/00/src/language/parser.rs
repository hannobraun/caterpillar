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
    type Item = SyntaxTreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        let mut arrays: Vec<Vec<_>> = Vec::new();

        for token in &mut self.tokens {
            match token {
                Token::Fn { name } => {
                    let syntax_tree = SyntaxTreeNode::Fn { name };
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
                        return Some(SyntaxTreeNode::Array { syntax_tree });
                    }

                    // If there's no array open, nothing will happen. We can
                    // make this stricter, later on.
                }
            }
        }

        None
    }
}

pub enum SyntaxTreeNode {
    /// A function
    Fn { name: String },

    /// An array
    Array { syntax_tree: Vec<SyntaxTreeNode> },
}
