use capi_runtime::{Evaluator, Functions};

pub struct Program {
    pub functions: Functions,
    pub evaluator: Evaluator,
    pub entry: usize,
}

impl Program {
    pub fn new() -> Self {
        let mut functions = Functions::new();

        functions.define("draw_to_frame_buffer", |s| {
            s.w("compute_frame_buffer_len")
                .w("frame_buffer_addr")
                .w("store_all_pixels")
                .w("drop2");
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
        let entry = code.symbols.resolve("draw_to_frame_buffer");

        let evaluator = Evaluator::new(code);

        Self {
            functions,
            evaluator,
            entry,
        }
    }

    pub fn run(
        &mut self,
        frame_width: usize,
        frame_height: usize,
        frame: &mut [u8],
    ) {
        self.evaluator.data_stack.push(frame_width);
        self.evaluator.data_stack.push(frame_height);
        self.evaluator.evaluate(self.entry, frame);

        assert_eq!(self.evaluator.data_stack.num_values(), 0);
    }
}
