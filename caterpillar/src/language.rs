use std::{cell::RefCell, rc::Rc};

pub fn init() -> (Interpreter, Output) {
    let background_color = Rc::new(RefCell::new([0., 0., 0., 1.]));
    let language = Interpreter {
        background_color: background_color.clone(),
    };

    (language, background_color)
}

pub struct Interpreter {
    pub background_color: Rc<RefCell<[f64; 4]>>,
}

impl Interpreter {
    pub fn interpret(&self, code: &str) {
        let Ok(value) = code.parse::<u8>() else {
            return
        };
        let value = value as f64 / u8::MAX as f64;

        *self.background_color.borrow_mut() = [value, value, value, 1.];
    }
}

pub type Output = Rc<RefCell<[f64; 4]>>;
