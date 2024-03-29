mod builtins;
mod data_stack;

use self::{
    builtins::{add, mul, store},
    data_stack::DataStack,
};

pub struct Lang<'r> {
    data_stack: DataStack,
    frame: &'r mut [u8],
}

impl<'r> Lang<'r> {
    pub fn new(frame: &'r mut [u8]) -> Self {
        Self {
            data_stack: DataStack::new(),
            frame,
        }
    }

    pub fn b(&mut self, name: &'static str) -> &mut Self {
        match name {
            "add" => add(&mut self.data_stack),
            "mul" => mul(&mut self.data_stack),
            "store" => store(&mut self.data_stack, self.frame),
            _ => panic!("Unknown builtin: `{name}`"),
        }

        self
    }

    pub fn v(&mut self, value: usize) -> &mut Self {
        self.data_stack.push(value);
        self
    }
}

pub fn lang(frame_width: usize, frame_height: usize, frame: &mut [u8]) {
    let mut lang = Lang::new(frame);

    lang.v(frame_width).v(frame_height);

    store_all_pixels(&mut lang);

    assert_eq!(lang.data_stack.num_values(), 0);
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
    }
}

fn compute_draw_buffer_len(lang: &mut Lang) {
    lang.b("mul").v(4).b("mul");
}

fn frame_addr(lang: &mut Lang) {
    lang.v(0);
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
