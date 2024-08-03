use std::collections::VecDeque;

use capi_process::{Bytecode, Effect, Process, Value};

use crate::{
    display,
    host::{GameEngineEffect, GameEngineHost, TILES_PER_AXIS},
    memory::Memory,
};

pub struct GameEngine {
    pub process: Process<GameEngineHost>,

    bytecode: Option<Bytecode>,
    arguments: [Value; 2],
    memory: Memory,
    input: VecDeque<u8>,
    random: VecDeque<i32>,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            process: Process::default(),
            bytecode: None,
            arguments: [Value::from(TILES_PER_AXIS as i32); 2],
            memory: Memory::default(),
            input: VecDeque::new(),
            random: VecDeque::new(),
        }
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn on_new_bytecode(&mut self, bytecode: Bytecode) {
        self.bytecode = Some(bytecode);
        self.reset();
    }

    pub fn on_input(&mut self, value: u8) {
        self.input.push_back(value);
    }

    pub fn push_random(&mut self, value: i32) -> bool {
        if self.random.len() >= 1024 {
            return false;
        }

        self.random.push_back(value);

        true
    }

    pub fn reset(&mut self) {
        self.memory = Memory::default();
        self.process.reset(self.arguments);
    }

    pub fn run_until_end_of_frame(&mut self, pixels: &mut [u8]) {
        let Some(bytecode) = &self.bytecode else {
            return;
        };

        while self.process.state().can_step() {
            self.process.step(bytecode);

            if let Some(effect) = self.process.state().first_unhandled_effect()
            {
                match effect {
                    Effect::Core(_) => {
                        // We can't handle any core effects, and we don't need
                        // to:
                        //
                        // - With the unhandled effect, the process can no
                        //   longer step, which means this loop is done.
                        // - The caller can see the unhandled effect and handle
                        //   it accordingly (by sending it to the debugger, for
                        //   example).
                        continue;
                    }
                    Effect::Host(GameEngineEffect::Load { address }) => {
                        let address: usize = (*address).into();
                        let value = self.memory.inner[address];
                        self.process.push([value]);

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
                            self.input.pop_front().unwrap_or(0).into();

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
                }
            }
        }
    }
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::new()
    }
}
