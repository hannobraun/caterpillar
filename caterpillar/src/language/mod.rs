mod evaluator;
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

        let chars = code.chars();
        let tokens = tokenizer::tokenize(chars, &mut token_buf);
        let mut operations = evaluator::evaluate(tokens);

        let r = parse_color_channel(&mut operations);
        let g = parse_color_channel(&mut operations);
        let b = parse_color_channel(&mut operations);

        if let (Some(r), Some(g), Some(b)) = (r, g, b) {
            *self.background_color.borrow_mut() = [r, g, b, 1.];
        }
    }
}

fn parse_color_channel(
    mut operations: impl Iterator<Item = evaluator::Operation>,
) -> Option<f64> {
    let operation = operations.next()?;
    let evaluator::Operation::Push(value) = operation;
    Some(value as f64 / u8::MAX as f64)
}

pub type Output = Rc<RefCell<[f64; 4]>>;
