mod builtins;
mod compiler;
mod data_stack;
mod evaluator;
mod functions;
mod symbols;
mod syntax;

use self::{evaluator::Evaluator, functions::Functions};

pub fn lang(frame_width: usize, frame_height: usize, frame: &mut [u8]) {
    let mut capi = Capi::new();

    capi.functions.define("inc_addr", |s| {
        s.v(1).w("add");
    });
    capi.functions.define("store_channel", |s| {
        s.w("store").w("inc_addr");
    });
    capi.functions.define("store_red", |s| {
        s.v(0).w("store_channel");
    });
    capi.functions.define("store_green", |s| {
        s.v(255).w("store_channel");
    });
    capi.functions.define("store_blue", |s| {
        s.v(0).w("store_channel");
    });
    capi.functions.define("store_alpha", |s| {
        s.v(255).w("store_channel");
    });
    capi.functions.define("store_pixel", |s| {
        s.w("store_red")
            .w("store_green")
            .w("store_blue")
            .w("store_alpha");
    });

    let (instructions, symbols) = capi.functions.compile();
    let store_pixel = symbols
        .resolve("store_pixel")
        .expect("Can't find function that was just defined.");

    let mut evaluator = Evaluator::new(instructions);

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
}

impl Capi {
    pub fn new() -> Self {
        Self {
            functions: Functions::new(),
        }
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
