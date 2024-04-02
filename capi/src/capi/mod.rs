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

    functions.define("draw_to_frame_buffer", |s| {
        s.w("compute_frame_buffer_len")
            .w("frame_buffer_addr")
            .w("store_all_pixels");
    });
    functions.define("compute_frame_buffer_len", |s| {
        s.w("mul").w("num_channels").w("mul");
    });
    functions.define("num_channels", |s| {
        s.v(4);
    });
    functions.define("frame_buffer_addr", |s| {
        s.v(0);
    });
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
    let draw_to_frame_buffer = code.symbols.resolve("draw_to_frame_buffer");

    let mut evaluator = Evaluator::new(code);

    evaluator.data_stack.push(frame_width);
    evaluator.data_stack.push(frame_height);
    evaluator.evaluate(draw_to_frame_buffer, frame);
    let _addr = evaluator.data_stack.pop();
    let _buffer_len = evaluator.data_stack.pop();

    assert_eq!(evaluator.data_stack.num_values(), 0);
}
