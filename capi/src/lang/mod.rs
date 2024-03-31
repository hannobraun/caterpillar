mod builtins;
mod compiler;
mod data_stack;
mod functions;
mod syntax;

use self::{
    compiler::{compile, Instruction},
    data_stack::DataStack,
    functions::Functions,
    syntax::Syntax,
};

pub fn lang(frame_width: usize, frame_height: usize, frame: &mut [u8]) {
    let mut capi = Capi::new();

    capi.define_function("inc_addr", |s| {
        s.v(1).w("add");
    });
    capi.define_function("store_channel", |s| {
        s.w("store").f("inc_addr");
    });
    capi.define_function("store_red", |s| {
        s.v(0).f("store_channel");
    });
    capi.define_function("store_green", |s| {
        s.v(255).f("store_channel");
    });
    capi.define_function("store_blue", |s| {
        s.v(0).f("store_channel");
    });
    capi.define_function("store_alpha", |s| {
        s.v(255).f("store_channel");
    });
    let store_pixel = capi.define_function("store_pixel", |s| {
        s.f("store_red")
            .f("store_green")
            .f("store_blue")
            .f("store_alpha");
    });

    store_all_pixels(frame_width, frame_height, store_pixel, &mut capi, frame);

    assert_eq!(capi.data_stack.num_values(), 0);
}

#[derive(Debug)]
pub struct Capi {
    instructions: Vec<Instruction>,
    functions: Functions,
    call_stack: Vec<usize>,
    data_stack: DataStack,
}

impl Capi {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            functions: Functions::new(),
            call_stack: Vec::new(),
            data_stack: DataStack::new(),
        }
    }

    pub fn define_function(
        &mut self,
        name: &'static str,
        f: impl FnOnce(&mut Syntax),
    ) -> usize {
        let address = self.instructions.len();

        let mut syntax = Vec::new();
        f(&mut Syntax::new(&self.functions, &mut syntax));
        compile(syntax, &self.functions, &mut self.instructions);

        self.functions.define(name, address);

        address
    }

    pub fn execute(&mut self, entry: usize, frame: &mut [u8]) {
        let mut current_instruction = entry;

        loop {
            let instruction = self.instructions[current_instruction];
            current_instruction += 1;

            match instruction {
                Instruction::CallBuiltin { name } => match name {
                    "add" => builtins::add(&mut self.data_stack),
                    "mul" => builtins::mul(&mut self.data_stack),
                    "store" => builtins::store(&mut self.data_stack, frame),
                    _ => panic!("Unknown builtin: `{name}`"),
                },
                Instruction::CallFunction { address } => {
                    self.call_stack.push(current_instruction);
                    current_instruction = address;
                }
                Instruction::PushValue(value) => self.data_stack.push(value),
                Instruction::Return => {
                    let Some(return_address) = self.call_stack.pop() else {
                        break;
                    };

                    current_instruction = return_address;
                }
            }
        }
    }
}

fn store_all_pixels(
    frame_width: usize,
    frame_height: usize,
    store_pixel: usize,
    capi: &mut Capi,
    frame: &mut [u8],
) {
    let buffer_len = compute_draw_buffer_len(frame_width, frame_height);

    let mut addr = frame_addr();

    loop {
        if addr >= buffer_len {
            break;
        }

        capi.data_stack.push(addr);
        capi.execute(store_pixel, frame);
        addr = capi.data_stack.pop();
    }
}

fn compute_draw_buffer_len(frame_width: usize, frame_height: usize) -> usize {
    frame_width * frame_height * 4
}

fn frame_addr() -> usize {
    0
}
