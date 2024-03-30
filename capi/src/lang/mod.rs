mod builtins;
mod compiler;
mod data_stack;

use self::{compiler::Compiler, data_stack::DataStack};

pub fn lang(frame_width: usize, frame_height: usize, frame: &mut [u8]) {
    let mut lang = Lang::new(frame);

    lang.define_function("inc_addr", |lang| {
        lang.compiler.v(1).b("add");
    });

    lang.data_stack.push(frame_width);
    lang.data_stack.push(frame_height);

    store_all_pixels(&mut lang);

    assert_eq!(lang.data_stack.num_values(), 0);
}

pub struct Lang<'r> {
    compiler: Compiler,
    data_stack: DataStack,
    frame: &'r mut [u8],
}

impl<'r> Lang<'r> {
    pub fn new(frame: &'r mut [u8]) -> Self {
        Self {
            compiler: Compiler::new(),
            data_stack: DataStack::new(),
            frame,
        }
    }

    pub fn define_function(
        &mut self,
        name: &'static str,
        f: impl FnOnce(&mut Lang),
    ) {
        let mut lang = Lang::new(self.frame);
        f(&mut lang);
        self.compiler
            .functions
            .insert(name, lang.compiler.fragments);
    }

    pub fn execute(&mut self) {
        for fragment in self.compiler.fragments.drain(..) {
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

#[derive(Copy, Clone)]
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
    store_red(&mut lang.compiler);
    store_green(&mut lang.compiler);
    store_blue(&mut lang.compiler);
    store_alpha(&mut lang.compiler);
}

fn store_red(c: &mut Compiler) {
    c.v(0);
    store_channel(c);
}

fn store_green(c: &mut Compiler) {
    c.v(255);
    store_channel(c);
}

fn store_blue(c: &mut Compiler) {
    c.v(0);
    store_channel(c);
}

fn store_alpha(c: &mut Compiler) {
    c.v(255);
    store_channel(c);
}

fn store_channel(c: &mut Compiler) {
    c.b("store").f("inc_addr");
}
