mod builtins;
mod compiler;
mod data_stack;
mod evaluator;
mod functions;
mod symbols;
mod syntax;

use self::{
    compiler::{compile, Instruction},
    evaluator::Evaluator,
    functions::Functions,
    symbols::Symbols,
    syntax::Syntax,
};

pub fn lang(frame_width: usize, frame_height: usize, frame: &mut [u8]) {
    let mut capi = Capi::new();

    capi.define_function("inc_addr", |s| {
        s.v(1).w("add");
    });
    capi.define_function("store_channel", |s| {
        s.w("store").w("inc_addr");
    });
    capi.define_function("store_red", |s| {
        s.v(0).w("store_channel");
    });
    capi.define_function("store_green", |s| {
        s.v(255).w("store_channel");
    });
    capi.define_function("store_blue", |s| {
        s.v(0).w("store_channel");
    });
    capi.define_function("store_alpha", |s| {
        s.v(255).w("store_channel");
    });
    let store_pixel = capi.define_function("store_pixel", |s| {
        s.w("store_red")
            .w("store_green")
            .w("store_blue")
            .w("store_alpha");
    });

    let mut evaluator = Evaluator::new(capi.instructions);

    store_all_pixels(
        frame_width,
        frame_height,
        store_pixel,
        &mut evaluator,
        frame,
    );

    assert_eq!(evaluator.data_stack.num_values(), 0);
}

#[derive(Debug)]
pub struct Capi {
    functions: Functions,
    instructions: Vec<Instruction>,
    symbols: Symbols,
}

impl Capi {
    pub fn new() -> Self {
        Self {
            functions: Functions::new(),
            instructions: Vec::new(),
            symbols: Symbols::new(),
        }
    }

    pub fn define_function(
        &mut self,
        name: &'static str,
        f: impl FnOnce(&mut Syntax),
    ) -> usize {
        let address = self.instructions.len();

        let mut syntax = Vec::new();
        f(&mut Syntax::new(&mut syntax));
        self.functions.define(name, syntax.clone());

        compile(syntax, &self.symbols, &mut self.instructions);

        self.symbols.define(name, address);

        address
    }
}

fn store_all_pixels(
    frame_width: usize,
    frame_height: usize,
    store_pixel: usize,
    evaluator: &mut Evaluator,
    frame: &mut [u8],
) {
    let buffer_len = compute_draw_buffer_len(frame_width, frame_height);

    let mut addr = frame_addr();

    loop {
        if addr >= buffer_len {
            break;
        }

        evaluator.data_stack.push(addr);
        evaluator.execute(store_pixel, frame);
        addr = evaluator.data_stack.pop();
    }
}

fn compute_draw_buffer_len(frame_width: usize, frame_height: usize) -> usize {
    frame_width * frame_height * 4
}

fn frame_addr() -> usize {
    0
}
