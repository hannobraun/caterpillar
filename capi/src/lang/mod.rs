mod builtins;
mod data_stack;

use self::data_stack::DataStack;

pub fn lang(frame_width: usize, frame_height: usize, frame: &mut [u8]) {
    let mut lang = Lang::new(frame);

    lang.data_stack.push(frame_width);
    lang.data_stack.push(frame_height);

    store_all_pixels(&mut lang);

    assert_eq!(lang.data_stack.num_values(), 0);
}

pub struct Lang<'r> {
    data_stack: DataStack,
    frame: &'r mut [u8],
    fragments: Vec<Fragment>,
}

impl<'r> Lang<'r> {
    pub fn new(frame: &'r mut [u8]) -> Self {
        Self {
            data_stack: DataStack::new(),
            frame,
            fragments: Vec::new(),
        }
    }

    pub fn b(&mut self, name: &'static str) -> &mut Self {
        self.fragments.push(Fragment::Builtin { name });
        self
    }

    pub fn v(&mut self, value: usize) -> &mut Self {
        self.fragments.push(Fragment::Value(value));
        self
    }

    pub fn execute(&mut self) {
        for fragment in self.fragments.drain(..) {
            match fragment {
                Fragment::Builtin { name } => match name {
                    "add" => builtins::add(&mut self.data_stack),
                    "mul" => builtins::mul(&mut self.data_stack),
                    "store" => {
                        builtins::store(&mut self.data_stack, self.frame)
                    }
                    _ => panic!("Unknown builtin: `{name}`"),
                },
                Fragment::Value(value) => self.data_stack.push(value),
            }
        }
    }
}

pub enum Fragment {
    Builtin { name: &'static str },
    Value(usize),
}

fn store_all_pixels(lang: &mut Lang) {
    compute_draw_buffer_len(lang);
    let buffer_len = lang.data_stack.pop();

    frame_addr(lang);

    loop {
        let addr = lang.data_stack.pop();
        if addr >= buffer_len {
            break;
        }
        lang.data_stack.push(addr);

        store_pixel(lang);
        lang.execute();
    }
}

fn compute_draw_buffer_len(lang: &mut Lang) {
    builtins::mul(&mut lang.data_stack);
    lang.data_stack.push(4);
    builtins::mul(&mut lang.data_stack);
}

fn frame_addr(lang: &mut Lang) {
    lang.data_stack.push(0);
}

fn store_pixel(lang: &mut Lang) {
    store_red(lang);
    store_green(lang);
    store_blue(lang);
    store_alpha(lang);
}

fn store_red(lang: &mut Lang) {
    lang.v(0);
    store_channel(lang);
}

fn store_green(lang: &mut Lang) {
    lang.v(255);
    store_channel(lang);
}

fn store_blue(lang: &mut Lang) {
    lang.v(0);
    store_channel(lang);
}

fn store_alpha(lang: &mut Lang) {
    lang.v(255);
    store_channel(lang);
}

fn store_channel(lang: &mut Lang) {
    lang.b("store");
    inc_addr(lang);
}

fn inc_addr(lang: &mut Lang) {
    lang.v(1).b("add");
}
