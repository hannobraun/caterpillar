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

pub type Output = Rc<RefCell<[f64; 4]>>;
