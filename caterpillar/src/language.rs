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
        let mut code = code.chars();

        let r = parse_color_channel(&mut code);
        let g = parse_color_channel(&mut code);
        let b = parse_color_channel(&mut code);

        if let (Some(r), Some(g), Some(b)) = (r, g, b) {
            *self.background_color.borrow_mut() = [r, g, b, 1.];
        }
    }
}

fn parse_color_channel(code: impl Iterator<Item = char>) -> Option<f64> {
    let mut word = String::new();
    read_token(code, &mut word);

    let Ok(value) = word.parse::<u8>() else {
        return None;
    };
    Some(value as f64 / u8::MAX as f64)
}

fn read_token(code: impl Iterator<Item = char>, word: &mut String) {
    // I think it would be a bit nicer to do this with `Iterator::collect_into`,
    // but that is not stable yet, as of this writing.
    word.extend(
        code.skip_while(|ch| ch.is_whitespace())
            .take_while(|ch| !ch.is_whitespace()),
    );
}

pub type Output = Rc<RefCell<[f64; 4]>>;
