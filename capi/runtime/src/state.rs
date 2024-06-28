use std::{collections::VecDeque, panic};

use capi_compiler::{compiler::compile, games::snake::snake, syntax::Script};
use capi_process::{BuiltinEffect, Code, EvaluatorEffect, Process, Value};
use capi_protocol::{
    command::{Command, SerializedCommand},
    memory::Memory,
};

use crate::{display, ffi, tiles::TILES_PER_AXIS, updates::Updates};

pub struct RuntimeState {
    pub code: Option<Code>,
    pub arguments: Vec<Value>,
    pub process: Process,
    pub memory: Memory,
    pub input: Input,
    pub random: VecDeque<i8>,
    pub commands: Vec<SerializedCommand>,
    pub updates: Updates,
}

impl RuntimeState {
    pub fn new() -> Self {
        panic::set_hook(Box::new(|panic_info| {
            // This _should_ be sound, because there _should_ be no other
            // reference to `ffi::PANIC` in existence right now, and we're
            // dropping this one before returning control to the host.
            //
            // Unless the code in `ffi` that handles panics is panicking, in
            // which case I don't think a little bit of unsoundness makes a
            // difference.
            let panic = unsafe { ffi::PANIC.access() };
            *panic = Some(panic_info.to_string());
        }));

        let mut process = Process::default();

        let mut script = Script::default();
        snake(&mut script);

        let (code, source_map) = compile(&script);

        let arguments = vec![Value(TILES_PER_AXIS as i8); 2];
        process.reset(&code, arguments.clone());

        let memory = Memory::default();
        let input = Input::default();
        let mut updates = Updates::new();

        updates.queue_source_code(script.functions, source_map);

        Self {
            code: Some(code),
            arguments,
            process,
            memory,
            input,
            commands: Vec::new(),
            random: VecDeque::new(),
            updates,
        }
    }

    pub fn update(&mut self, pixels: &mut [u8]) {
        let Some(code) = &self.code else {
            return;
        };

        for command in self.commands.drain(..) {
            let command = Command::deserialize(command);

            match command {
                Command::BreakpointClear { location } => {
                    self.process.clear_durable_breakpoint(location);
                }
                Command::BreakpointSet { location } => {
                    self.process.set_durable_breakpoint(location);
                }
                Command::Continue { and_stop_at } => {
                    self.process.continue_(and_stop_at);
                }
                Command::Reset => {
                    self.process.reset(code, self.arguments.clone());
                    self.memory = Memory::default();
                }
                Command::Step => {
                    if let Some(EvaluatorEffect::Builtin(
                        BuiltinEffect::Breakpoint,
                    )) = self.process.state().first_unhandled_effect()
                    {
                        let and_stop_at = self
                            .process
                            .stack()
                            .next_instruction_overall()
                            .unwrap();
                        self.process.continue_(Some(and_stop_at))
                    }
                }
                Command::Stop => {
                    self.process.stop();
                }
            }
        }

        while self.process.state().can_step() {
            self.process.step(code);

            if let Some(EvaluatorEffect::Builtin(effect)) =
                self.process.state().first_unhandled_effect()
            {
                match effect {
                    BuiltinEffect::Breakpoint => {
                        // Nothing to do here. With an unhandled effect, the
                        // program won't continue running. The debugger is in
                        // control of what happens next.
                    }
                    BuiltinEffect::Error(_) => {
                        // Nothing needs to be done. With an unhandled
                        // effect, the program won't continue running, and
                        // the debugger will see the error and display it.
                    }
                    BuiltinEffect::Load { address } => {
                        let address: usize = (*address).into();
                        let value = self.memory.inner[address];
                        self.process.push([value]);

                        self.process.handle_first_effect();
                    }
                    BuiltinEffect::Store { address, value } => {
                        let address: usize = (*address).into();
                        self.memory.inner[address] = *value;

                        self.process.handle_first_effect();
                    }
                    BuiltinEffect::SetTile { x, y, color } => {
                        let x = *x;
                        let y = *y;
                        let color = *color;

                        self.process.handle_first_effect();

                        display::set_tile(x.into(), y.into(), color, pixels);
                    }
                    BuiltinEffect::SubmitFrame => {
                        // This effect means that the game is done rendering.
                        // Let's break out of this loop now, so we can do our
                        // part in that and return control to the host.
                        self.process.handle_first_effect();
                        break;
                    }
                    BuiltinEffect::ReadInput => {
                        let input = self
                            .input
                            .buffer
                            .pop_front()
                            .unwrap_or(0)
                            .try_into()
                            .unwrap();

                        self.process.push([Value(input)]);
                        self.process.handle_first_effect();
                    }
                    BuiltinEffect::ReadRandom => {
                        // We get a lot of random numbers from the host, and
                        // they are topped off every frame. It should be a
                        // while, before Caterpillar programs become complex
                        // enough to run into this limit.
                        //
                        // If that happens, and we hit this `unwrap`, we can of
                        // course just increase the limit. But long-term, it
                        // probably makes more sense to implement a PRNG, either
                        // in Rust or Caterpillar, and only seed that with
                        // randomness from the host.
                        let random = self.random.pop_front().unwrap();

                        self.process.push([Value(random)]);
                        self.process.handle_first_effect();
                    }
                }
            }
        }

        self.updates.queue_updates(&self.process, &self.memory);
    }
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct Input {
    pub buffer: VecDeque<u8>,
}
