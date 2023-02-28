mod evaluator;
mod parser;
mod tokenizer;
mod values;

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
        let mut stack = evaluator::Stack::new();

        let mut chars = code.chars();
        let mut tokens = tokenizer::Tokenizer::new(&mut chars);
        let syntax_tree = parser::Parser::new(&mut tokens);
        evaluator::evaluate(syntax_tree, &mut stack);

        let Some(values::Value::Color(color)) = stack.pop_front() else {
            return;
        };

        let r = parse_color_channel(color.r);
        let g = parse_color_channel(color.g);
        let b = parse_color_channel(color.b);

        if let (Some(r), Some(g), Some(b)) = (r, g, b) {
            *self.background_color.borrow_mut() = [r, g, b, 1.];
        }
    }
}

fn parse_color_channel(value: u8) -> Option<f64> {
    Some(value as f64 / u8::MAX as f64)
}

pub type Output = Rc<RefCell<[f64; 4]>>;
