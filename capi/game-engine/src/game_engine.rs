use std::collections::VecDeque;

use capi_process::{Effect, Instructions, Process, Value};

use crate::{
    display,
    host::{GameEngineEffect, GameEngineHost, TILES_PER_AXIS},
    memory::Memory,
};

pub struct GameEngine {
    pub process: Process<GameEngineHost>,

    instructions: Option<Instructions>,
    arguments: [Value; 2],
    memory: Memory,
    input: VecDeque<u8>,
    random: VecDeque<i32>,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            process: Process::default(),
            instructions: None,
            arguments: [Value::from(TILES_PER_AXIS as i32); 2],
            memory: Memory::default(),
            input: VecDeque::new(),
            random: VecDeque::new(),
        }
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn on_new_bytecode(&mut self, instructions: Instructions) {
        self.instructions = Some(instructions);
        self.reset();
    }

    pub fn on_input(&mut self, value: u8) {
        self.input.push_back(value);
    }

    /// # Top off the game engine's random numbers
    ///
    /// Whatever code embeds `GameEngine` is expected to call this in a loop
    /// every frame, until this function returns `false`.
    ///
    /// ## Implementation Note
    ///
    /// It would make a lot more sense to expect a random seed in the
    /// constructor, and have a PRNG to get more randomness as needed.
    ///
    /// But this works too, and it's fine for now. Ideally, I would like to
    /// write the PRNG in Caterpillar, and it's a bit too early for that. I'm
    /// not in hurry to replace this with a Rust-based solution right now.
    /// We get a lot of random numbers from the host, and
    pub fn push_random(&mut self, value: i32) -> bool {
        // If games grow complex enough to need more than this many random
        // numbers per frame, we can increase this limit. But hopefully, we'll
        // have a PRNG by then.
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
        let Some(instructions) = &self.instructions else {
            return;
        };

        while self.process.state().can_step() {
            self.process.step(instructions);

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

                    Effect::Host(GameEngineEffect::SubmitFrame) => {
                        // The game is done rendering. This is our sign to break
                        // out of this loop.
                        //
                        // Other than that, there's nothing to do. We already
                        // updates the `pixels` argument, according to what the
                        // game was drawing. Lower-level code will take care of
                        // it from here.
                        self.process.handle_first_effect();
                        break;
                    }

                    Effect::Host(GameEngineEffect::Load { address }) => {
                        let address = match address.to_u8() {
                            Ok(address) => address,
                            Err(new_effect) => {
                                self.process.trigger_effect(new_effect);
                                continue;
                            }
                        };
                        let address: usize = address.into();

                        let value = self.memory.inner[address];
                        self.process.push([value]);
                    }
                    Effect::Host(GameEngineEffect::Store {
                        address,
                        value,
                    }) => {
                        let address: usize = (*address).into();
                        self.memory.inner[address] = *value;
                    }
                    Effect::Host(GameEngineEffect::ReadInput) => {
                        let input = self.input.pop_front().unwrap_or(0);
                        self.process.push([input]);
                    }
                    Effect::Host(GameEngineEffect::ReadRandom) => {
                        // See `GameEngine::push_random` for context.
                        let random = self.random.pop_front().unwrap();
                        self.process.push([random]);
                    }
                    Effect::Host(GameEngineEffect::SetTile { x, y, color }) => {
                        let x = *x;
                        let y = *y;
                        let color = *color;

                        display::set_tile(x.into(), y.into(), color, pixels);
                    }
                }

                self.process.handle_first_effect();
            }
        }
    }
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::new()
    }
}
