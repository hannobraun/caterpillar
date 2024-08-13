use std::collections::VecDeque;

use capi_process::{Effect, Instructions, Process, Value};

use crate::{
    display::{self, TILES_PER_AXIS},
    host::GameEngineEffect,
    memory::Memory,
};

pub struct GameEngine {
    pub process: Process,

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
            arguments: [Value::from(TILES_PER_AXIS); 2],
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
        while self.process.can_step() {
            let Some(instructions) = &self.instructions else {
                return;
            };

            self.process.step(instructions);

            if let Some(effect) = self.process.effects_mut().handle_first() {
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
                        self.process.effects_mut().trigger(effect);
                    }
                    Err(new_effect) => {
                        self.process.effects_mut().trigger(effect);
                        self.process.effects_mut().trigger(new_effect);
                    }
                }
            }
        }
    }

    fn handle_effect(
        &mut self,
        effect: &Effect,
        pixels: &mut [u8],
    ) -> Result<EffectOutcome, Effect> {
        let host_effect = match effect {
            Effect::Host => {
                let effect = self.process.stack_mut().pop_operand()?;
                let effect =
                    effect.to_u8().map_err(|_| Effect::InvalidHostEffect)?;
                GameEngineEffect::try_from(effect)
                    .map_err(|_| Effect::InvalidHostEffect)?
            }
            _ => {
                return Ok(EffectOutcome::Unhandled);
            }
        };

        match host_effect {
            GameEngineEffect::SubmitFrame => {
                return Ok(EffectOutcome::WasSubmit);
            }

            GameEngineEffect::Load => {
                let address = self.process.stack_mut().pop_operand()?;

                let address = address.to_u8()?;

                let address: usize = address.into();
                let value = self.memory.inner[address];

                self.process.stack_mut().push_operand(value);
            }
            GameEngineEffect::Store => {
                let address = self.process.stack_mut().pop_operand()?;
                let value = self.process.stack_mut().pop_operand()?;

                let address = address.to_u8()?;
                let value = value.to_u8()?;

                let address: usize = address.into();
                self.memory.inner[address] = value;
            }
            GameEngineEffect::ReadInput => {
                let input = self.input.pop_front().unwrap_or(0);
                self.process.stack_mut().push_operand(input);
            }
            GameEngineEffect::ReadRandom => {
                // See `GameEngine::push_random` for context.
                let random = self.random.pop_front().unwrap();
                self.process.stack_mut().push_operand(random);
            }
            GameEngineEffect::SetPixel => {
                let a = self.process.stack_mut().pop_operand()?;
                let b = self.process.stack_mut().pop_operand()?;
                let g = self.process.stack_mut().pop_operand()?;
                let r = self.process.stack_mut().pop_operand()?;
                let y = self.process.stack_mut().pop_operand()?;
                let x = self.process.stack_mut().pop_operand()?;

                let x = x.to_u8()?;
                let y = y.to_u8()?;
                let r = r.to_u8()?;
                let g = g.to_u8()?;
                let b = b.to_u8()?;
                let a = a.to_u8()?;

                if x >= TILES_PER_AXIS {
                    return Err(Effect::OperandOutOfBounds);
                }
                if y >= TILES_PER_AXIS {
                    return Err(Effect::OperandOutOfBounds);
                }

                display::set_pixel(x.into(), y.into(), [r, g, b, a], pixels);
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
