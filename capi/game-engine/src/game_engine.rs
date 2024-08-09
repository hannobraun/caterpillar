use std::collections::VecDeque;

use capi_process::{CoreEffect, Effect, Instructions, Process, Value};

use crate::{
    display,
    host::{
        GameEngineEffect, GameEngineHost, TILES_PER_AXIS, TILES_PER_AXIS_I32,
    },
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
        while self.process.state().can_step() {
            let Some(instructions) = &self.instructions else {
                return;
            };

            self.process.step(instructions);

            if let Some(effect) = self.process.handle_first_effect() {
                match self.handle_effect(&effect, pixels) {
                    Ok(EffectOutcome::Handled) => {}
                    Ok(EffectOutcome::WasSubmit) => {
                        // The game is done rendering. This is our sign to break
                        // out of this loop.
                        //
                        // Other than that, there's nothing to do. We already
                        // updated the `pixels` argument, according to what the
                        // game was drawing. Lower-level code will take care of
                        // it from here.
                        break;
                    }
                    Ok(EffectOutcome::Unhandled) => {
                        self.process.trigger_effect(effect);
                    }
                    Err(new_effect) => {
                        self.process.trigger_effect(effect);
                        self.process.trigger_effect(new_effect);
                    }
                }
            }
        }
    }

    fn handle_effect(
        &mut self,
        effect: &Effect<GameEngineEffect>,
        pixels: &mut [u8],
    ) -> Result<EffectOutcome, Effect<GameEngineEffect>> {
        match effect {
            Effect::Core(_) => {
                // We can't handle any core effects, and we don't need to:
                //
                // - With the unhandled effect, the process can no longer step,
                //   which means this loop is done.
                // - The caller can see the unhandled effect and handle it
                //   accordingly (by sending it to the debugger, for example).
                return Ok(EffectOutcome::Unhandled);
            }

            Effect::Host(GameEngineEffect::SubmitFrame) => {
                return Ok(EffectOutcome::WasSubmit);
            }

            Effect::Host(GameEngineEffect::Load) => {
                let address = self.process.stack_mut().pop_operand()?;

                let address = address.to_u8()?;
                let address: usize = address.into();

                let value = self.memory.inner[address];
                self.process.stack_mut().push_operand(value);
            }
            Effect::Host(GameEngineEffect::Store { address, value }) => {
                let address: usize = (*address).into();
                self.memory.inner[address] = *value;
            }
            Effect::Host(GameEngineEffect::ReadInput) => {
                let input = self.input.pop_front().unwrap_or(0);
                self.process.stack_mut().push_operand(input);
            }
            Effect::Host(GameEngineEffect::ReadRandom) => {
                // See `GameEngine::push_random` for context.
                let random = self.random.pop_front().unwrap();
                self.process.stack_mut().push_operand(random);
            }
            Effect::Host(GameEngineEffect::SetTile) => {
                let a = self.process.stack_mut().pop_operand()?;
                let b = self.process.stack_mut().pop_operand()?;
                let g = self.process.stack_mut().pop_operand()?;
                let r = self.process.stack_mut().pop_operand()?;
                let y = self.process.stack_mut().pop_operand()?;
                let x = self.process.stack_mut().pop_operand()?;

                let x = i32::from_le_bytes(x.0);
                let y = i32::from_le_bytes(y.0);
                let r = i32::from_le_bytes(r.0);
                let g = i32::from_le_bytes(g.0);
                let b = i32::from_le_bytes(b.0);
                let a = i32::from_le_bytes(a.0);

                if x < 0 {
                    return Err(Effect::Core(CoreEffect::OperandOutOfBounds));
                }
                if y < 0 {
                    return Err(Effect::Core(CoreEffect::OperandOutOfBounds));
                }
                if x >= TILES_PER_AXIS_I32 {
                    return Err(Effect::Core(CoreEffect::OperandOutOfBounds));
                }
                if y >= TILES_PER_AXIS_I32 {
                    return Err(Effect::Core(CoreEffect::OperandOutOfBounds));
                }

                let color_channel_min: i32 = u8::MIN.into();
                let color_channel_max: i32 = u8::MAX.into();

                if r < color_channel_min {
                    return Err(Effect::Core(CoreEffect::OperandOutOfBounds));
                }
                if g < color_channel_min {
                    return Err(Effect::Core(CoreEffect::OperandOutOfBounds));
                }
                if b < color_channel_min {
                    return Err(Effect::Core(CoreEffect::OperandOutOfBounds));
                }
                if a < color_channel_min {
                    return Err(Effect::Core(CoreEffect::OperandOutOfBounds));
                }
                if r > color_channel_max {
                    return Err(Effect::Core(CoreEffect::OperandOutOfBounds));
                }
                if r > color_channel_max {
                    return Err(Effect::Core(CoreEffect::OperandOutOfBounds));
                }
                if r > color_channel_max {
                    return Err(Effect::Core(CoreEffect::OperandOutOfBounds));
                }
                if r > color_channel_max {
                    return Err(Effect::Core(CoreEffect::OperandOutOfBounds));
                }

                let [x, y]: [u8; 2] = [x, y].map(|coord| {
                    coord.try_into().expect(
                        "Just checked that coordinates are within bounds",
                    )
                });
                let color = [r, g, b, a].map(|channel| {
                    channel.try_into().expect(
                        "Just checked that color channels are within bounds",
                    )
                });

                display::set_tile(x.into(), y.into(), color, pixels);
            }
        }

        Ok(EffectOutcome::Handled)
    }
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::new()
    }
}

enum EffectOutcome {
    Handled,
    WasSubmit,
    Unhandled,
}
