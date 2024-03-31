mod builtins;
mod compiler;
mod data_stack;

use self::{
    compiler::{Compiler, Functions, Instruction},
    data_stack::DataStack,
};

pub fn lang(frame_width: usize, frame_height: usize, frame: &mut [u8]) {
    let mut lang = Lang::new(frame);

    lang.define_function("inc_addr", |c| {
        c.v(1).b("add");
    });
    lang.define_function("store_channel", |c| {
        c.b("store").f("inc_addr");
    });
    lang.define_function("store_red", |c| {
        c.v(0).f("store_channel");
    });
    lang.define_function("store_green", |c| {
        c.v(255).f("store_channel");
    });
    lang.define_function("store_blue", |c| {
        c.v(0).f("store_channel");
    });
    lang.define_function("store_alpha", |c| {
        c.v(255).f("store_channel");
    });
    lang.define_function("store_pixel", |c| {
        c.f("store_red")
            .f("store_green")
            .f("store_blue")
            .f("store_alpha");
    });

    store_all_pixels(frame_width, frame_height, &mut lang);

    assert_eq!(lang.data_stack.num_values(), 0);
}

#[derive(Debug)]
pub struct Lang<'r> {
    compiler: Compiler,
    call_stack: Vec<usize>,
    data_stack: DataStack,
    frame: &'r mut [u8],
}

impl<'r> Lang<'r> {
    pub fn new(frame: &'r mut [u8]) -> Self {
        Self {
            compiler: Compiler::new(Functions::new()),
            call_stack: Vec::new(),
            data_stack: DataStack::new(),
            frame,
        }
    }

    pub fn define_function(
        &mut self,
        name: &'static str,
        f: impl FnOnce(&mut Compiler),
    ) {
        let mut compiler = Compiler::new(self.compiler.functions.clone());
        f(&mut compiler);
        self.compiler.functions.insert(name, compiler.instructions);
    }

    pub fn execute(&mut self, entry: usize) {
        let mut current_instruction = entry;

        loop {
            let instruction = self.compiler.instructions[current_instruction];
            current_instruction += 1;

            match instruction {
                Instruction::CallBuiltin { name } => match name {
                    "add" => builtins::add(&mut self.data_stack),
                    "mul" => builtins::mul(&mut self.data_stack),
                    "store" => {
                        builtins::store(&mut self.data_stack, self.frame)
                    }
                    _ => panic!("Unknown builtin: `{name}`"),
                },
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

fn store_all_pixels(frame_width: usize, frame_height: usize, lang: &mut Lang) {
    let buffer_len = compute_draw_buffer_len(frame_width, frame_height);

    let mut addr = frame_addr();

    loop {
        if addr >= buffer_len {
            break;
        }

        lang.data_stack.push(addr);
        lang.compiler.f("store_pixel");
        lang.compiler.instructions.push(Instruction::Return);
        lang.execute(0);
        lang.compiler.instructions.clear();
        addr = lang.data_stack.pop();
    }
}

fn compute_draw_buffer_len(frame_width: usize, frame_height: usize) -> usize {
    frame_width * frame_height * 4
}

fn frame_addr() -> usize {
    0
}
