use std::{cell::RefCell, rc::Rc};

pub fn init() -> (Language, Rc<RefCell<[f64; 4]>>) {
    let background_color = Rc::new(RefCell::new([0., 0., 0., 1.]));
    let language = Language::new(&background_color);
    (language, background_color)
}

pub struct Language {
    pub background_color: Rc<RefCell<[f64; 4]>>,
}

impl Language {
    pub fn new(background_color: &Rc<RefCell<[f64; 4]>>) -> Self {
        let background_color = background_color.clone();
        Self { background_color }
    }
}
