mod tokenizer;

use std::{cell::RefCell, rc::Rc};

pub fn init() -> (Interpreter, Output) {
    let background_color = Rc::new(RefCell::new([0., 0., 0., 1.]));
    let language = Interpreter {
        background_color: background_color.clone(),
    };

    (language, background_color)
}

pub struct Interpreter {
    background_color: Rc<RefCell<[f64; 4]>>,
}

impl Interpreter {
    pub fn interpret(&self, code: &str) {
        let mut token_buf = tokenizer::Buf::new();

        let mut chars = code.chars();
        let mut tokens = tokenizer::tokenize(&mut chars, &mut token_buf);

        let r = parse_color_channel(&mut tokens);
        let g = parse_color_channel(&mut tokens);
        let b = parse_color_channel(&mut tokens);

        if let (Some(r), Some(g), Some(b)) = (r, g, b) {
            *self.background_color.borrow_mut() = [r, g, b, 1.];
        }
    }
}

fn parse_color_channel(
    mut tokens: impl Iterator<Item = tokenizer::Token>,
) -> Option<f64> {
    let tokenizer::Token::Fn(function) = tokens.next()?;
    let Ok(value) = function.parse::<u8>() else {
        return None;
    };
    Some(value as f64 / u8::MAX as f64)
}

pub type Output = Rc<RefCell<[f64; 4]>>;
