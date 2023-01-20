use std::{cell::RefCell, rc::Rc};

pub fn init() -> (Language, Output) {
    let background_color = Rc::new(RefCell::new([0., 0., 0., 1.]));
    let language = Language {
        background_color: background_color.clone(),
    };

    (language, background_color)
}

pub struct Language {
    pub background_color: Rc<RefCell<[f64; 4]>>,
}

pub type Output = Rc<RefCell<[f64; 4]>>;
