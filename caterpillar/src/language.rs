use std::{cell::RefCell, rc::Rc};

pub struct Language {
    pub background_color: Rc<RefCell<[f64; 4]>>,
}
