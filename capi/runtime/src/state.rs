use std::{collections::VecDeque, panic};

use capi_game_engine::{
    display,
    host::{GameEngineEffect, GameEngineHost, TILES_PER_AXIS},
    input::Input,
    memory::Memory,
};
use capi_process::{Bytecode, CoreEffect, Effect, Process, Value};
use capi_protocol::{
    command::{Command, SerializedCommand},
    updates::Updates,
};

use crate::ffi_out::on_panic;

pub struct RuntimeState {
    pub bytecode: Option<Bytecode>,
    pub arguments: Vec<Value>,
    pub process: Process<GameEngineHost>,
    pub memory: Memory,
    pub input: Input,
    pub random: VecDeque<i32>,
    pub commands: Vec<SerializedCommand>,
    pub updates: Updates<GameEngineHost>,
}

impl RuntimeState {
    pub fn new() -> Self {
        panic::set_hook(Box::new(|panic_info| {
            on_panic(&panic_info.to_string());
        }));

        let arguments = vec![Value((TILES_PER_AXIS as i32).to_le_bytes()); 2];
        let process = Process::default();
        let memory = Memory::default();
        let input = Input::default();
        let updates = Updates::default();

        Self {
            bytecode: None,
            arguments,
            process,
            memory,
            input,
            commands: Vec::new(),
            random: VecDeque::new(),
            updates,
        }
    }

    pub fn on_new_bytecode(&mut self, bytecode: Bytecode) {
        self.process.reset(self.arguments.clone());
        self.bytecode = Some(bytecode);
    }

    pub fn update(&mut self, pixels: &mut [u8]) {
        let Some(bytecode) = &self.bytecode else {
            return;
        };

        for command in self.commands.drain(..) {
            let command = Command::deserialize(command);

            match command {
                Command::BreakpointClear { instruction } => {
                    self.process.clear_durable_breakpoint(&instruction);
                }
                Command::BreakpointSet { instruction } => {
                    self.process.set_durable_breakpoint(instruction);
                }
                Command::Continue { and_stop_at } => {
                    self.process.continue_(and_stop_at);
                }
                Command::Reset => {
                    self.process.reset(self.arguments.clone());
                    self.memory = Memory::default();
                }
                Command::Step => {
                    if let Some(Effect::Core(CoreEffect::Breakpoint)) =
                        self.process.state().first_unhandled_effect()
                    {
                        let and_stop_at =
                            self.process.stack().next_instruction();
                        self.process.continue_(Some(and_stop_at))
                    } else {
                        // If we're not stopped at a breakpoint, we can't step.
                        // It would be better, if this resulted in an explicit
                        // error that is sent to the debugger, instead of
                        // silently being ignored here.
                    }
                }
                Command::Stop => {
                    self.process.stop();
                }
            }
        }

        while self.process.state().can_step() {
            self.process.step(bytecode);

            if let Some(effect) = self.process.state().first_unhandled_effect()
            {
                match effect {
                    Effect::Core(CoreEffect::Breakpoint) => {
                        // Nothing to do here. With an unhandled effect, the
                        // program won't continue running. The debugger is in
                        // control of what happens next.
                    }
                    Effect::Host(GameEngineEffect::Load { address }) => {
                        let address: usize = (*address).into();
                        let value = self.memory.inner[address];
                        let value: i32 = value.into();
                        self.process.push([Value(value.to_le_bytes())]);

                        self.process.handle_first_effect();
                    }
                    Effect::Host(GameEngineEffect::Store {
                        address,
                        value,
                    }) => {
                        let address: usize = (*address).into();
                        self.memory.inner[address] = *value;

                        self.process.handle_first_effect();
                    }
                    Effect::Host(GameEngineEffect::SetTile { x, y, color }) => {
                        let x = *x;
                        let y = *y;
                        let color = *color;

                        self.process.handle_first_effect();

                        display::set_tile(x.into(), y.into(), color, pixels);
                    }
                    Effect::Host(GameEngineEffect::SubmitFrame) => {
                        // This effect means that the game is done rendering.
                        // Let's break out of this loop now, so we can do our
                        // part in that and return control to the host.
                        self.process.handle_first_effect();
                        break;
                    }
                    Effect::Host(GameEngineEffect::ReadInput) => {
                        let input: i32 =
                            self.input.buffer.pop_front().unwrap_or(0).into();

                        self.process.push([Value(input.to_le_bytes())]);
                        self.process.handle_first_effect();
                    }
                    Effect::Host(GameEngineEffect::ReadRandom) => {
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

                        self.process.push([Value(random.to_le_bytes())]);
                        self.process.handle_first_effect();
                    }
                    _ => {
                        // Nothing needs to be done. With an unhandled
                        // effect, the program won't continue running, and
                        // the debugger will see the error and display it.
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
