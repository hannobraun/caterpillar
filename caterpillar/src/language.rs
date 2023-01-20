use std::{cell::RefCell, rc::Rc};

pub struct Language {
    pub background_color: Rc<RefCell<[f64; 4]>>,
}

impl Language {
    pub fn new(background_color: &Rc<RefCell<[f64; 4]>>) -> Self {
        let background_color = background_color.clone();
        Self { background_color }
    }
}
