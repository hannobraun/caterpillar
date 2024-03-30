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
        c.f("store_red");
        c.f("store_green");
        c.f("store_blue");
        c.f("store_alpha");
    });

    lang.data_stack.push(frame_width);
    lang.data_stack.push(frame_height);

    store_all_pixels(&mut lang);

    assert_eq!(lang.data_stack.num_values(), 0);
}

#[derive(Debug)]
pub struct Lang<'r> {
    compiler: Compiler,
    data_stack: DataStack,
    frame: &'r mut [u8],
}

impl<'r> Lang<'r> {
    pub fn new(frame: &'r mut [u8]) -> Self {
        Self {
            compiler: Compiler::new(Functions::new()),
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

    pub fn execute(&mut self) {
        let mut current_instruction = 0;

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
                    break;
                }
            }
        }
    }
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

        lang.compiler.f("store_pixel");
        lang.compiler.instructions.push(Instruction::Return);
        lang.execute();
        lang.compiler.instructions.clear();
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
