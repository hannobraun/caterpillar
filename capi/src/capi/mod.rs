mod builtins;
mod code;
mod compiler;
mod data_stack;
mod evaluator;
mod functions;
mod symbols;
mod syntax;

use self::{evaluator::Evaluator, functions::Functions};

pub fn capi(frame_width: usize, frame_height: usize, frame: &mut [u8]) {
    let mut functions = Functions::new();

    functions.define("store_all_pixels", |s| {
        s.w("clone2")
            .w("sub")
            .w("return_if_zero")
            .w("store_pixel")
            .w("store_all_pixels");
    });
    functions.define("store_pixel", |s| {
        s.w("store_red")
            .w("store_green")
            .w("store_blue")
            .w("store_alpha");
    });
    functions.define("store_red", |s| {
        s.v(0).w("store_channel");
    });
    functions.define("store_green", |s| {
        s.v(255).w("store_channel");
    });
    functions.define("store_blue", |s| {
        s.v(0).w("store_channel");
    });
    functions.define("store_alpha", |s| {
        s.v(255).w("store_channel");
    });
    functions.define("store_channel", |s| {
        s.w("store").w("inc_addr");
    });
    functions.define("inc_addr", |s| {
        s.v(1).w("add");
    });

    let code = functions.compile();
    let entry = code.symbols.resolve("store_all_pixels");

    let mut evaluator = Evaluator::new(code);

    draw_to_frame_buffer(
        frame_width,
        frame_height,
        entry,
        &mut evaluator,
        frame,
    );

    assert_eq!(evaluator.data_stack.num_values(), 0);
}

fn draw_to_frame_buffer(
    frame_width: usize,
    frame_height: usize,
    store_all_pixels: usize,
    evaluator: &mut Evaluator,
    frame: &mut [u8],
) {
    let buffer_len = compute_frame_buffer_len(frame_width, frame_height);
    let addr = frame_buffer_addr();

    evaluator.data_stack.push(buffer_len);
    evaluator.data_stack.push(addr);
    evaluator.evaluate(store_all_pixels, frame);
    let _addr = evaluator.data_stack.pop();
    let _buffer_len = evaluator.data_stack.pop();
}

fn compute_frame_buffer_len(frame_width: usize, frame_height: usize) -> usize {
    frame_width * frame_height * 4
}

fn frame_buffer_addr() -> usize {
    0
}
