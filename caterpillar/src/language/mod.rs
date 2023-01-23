mod evaluator;
mod executor;
mod tokenizer;
mod values;

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
        let mut stack = VecDeque::new();

        let chars = code.chars();
        let tokens = tokenizer::tokenize(chars, &mut token_buf);
        let ops = evaluator::evaluate(tokens);
        executor::execute(ops, &mut stack);

        let r = parse_color_channel(&mut stack);
        let g = parse_color_channel(&mut stack);
        let b = parse_color_channel(&mut stack);

        if let (Some(r), Some(g), Some(b)) = (r, g, b) {
            *self.background_color.borrow_mut() = [r, g, b, 1.];
        }
    }
}

fn parse_color_channel(stack: &mut VecDeque<values::Value>) -> Option<f64> {
    let values::Value::U8(value) = stack.pop_front()?;
    Some(value as f64 / u8::MAX as f64)
}

pub type Output = Rc<RefCell<[f64; 4]>>;
