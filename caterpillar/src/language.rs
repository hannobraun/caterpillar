use std::{cell::RefCell, iter, rc::Rc};

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
        let mut tokens = tokenize(&mut code);

        let r = parse_color_channel(&mut tokens);
        let g = parse_color_channel(&mut tokens);
        let b = parse_color_channel(&mut tokens);

        if let (Some(r), Some(g), Some(b)) = (r, g, b) {
            *self.background_color.borrow_mut() = [r, g, b, 1.];
        }
    }
}

pub fn tokenize(
    code: &mut impl Iterator<Item = char>,
) -> impl Iterator<Item = String> + '_ {
    iter::from_fn(|| {
        let mut token = String::new();
        token.extend(
            code.by_ref()
                .skip_while(|ch| ch.is_whitespace())
                .take_while(|ch| !ch.is_whitespace()),
        );

        if token.is_empty() {
            return None;
        }

        Some(token)
    })
}

fn parse_color_channel(
    mut tokens: impl Iterator<Item = String>,
) -> Option<f64> {
    let token = tokens.next()?;
    let Ok(value) = token.parse::<u8>() else {
        return None;
    };
    Some(value as f64 / u8::MAX as f64)
}

pub type Output = Rc<RefCell<[f64; 4]>>;
