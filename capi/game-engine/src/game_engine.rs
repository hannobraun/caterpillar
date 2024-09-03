use std::collections::VecDeque;

use capi_process::{Effect, Instructions, Process, Value};

use crate::{
    display::{self, TILES_PER_AXIS},
    host::GameEngineEffect,
    memory::Memory,
};

pub struct GameEngine {
    pub process: Process,

    acc_time_s: f64,
    last_frame_start_s: Option<f64>,
    instructions: Option<Instructions>,
    arguments: [Value; 2],
    memory: Memory,
    input: VecDeque<u8>,
    random: VecDeque<i32>,
}

impl GameEngine {
    pub fn new() -> Self {
        let mut process = Process::default();
        let arguments = [Value::from(TILES_PER_AXIS); 2];

        process.reset(arguments);

        Self {
            process,
            acc_time_s: 0.,
            last_frame_start_s: None,
            instructions: None,
            arguments,
            memory: Memory::default(),
            input: VecDeque::new(),
            random: VecDeque::new(),
        }
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn on_new_instructions(&mut self, instructions: Instructions) {
        self.instructions = Some(instructions);
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

    pub fn run_until_end_of_frame(
        &mut self,
        delta_time_s: f64,
        current_time_s: f64,
        pixels: &mut [u8],
    ) -> bool {
        // For now, we're targeting an unambitious 30 fps.
        let frame_time_s = 1. / 30.;

        self.acc_time_s += delta_time_s;

        if self.acc_time_s >= frame_time_s {
            // It's time to run another frame!
            self.acc_time_s -= frame_time_s;
            self.last_frame_start_s = Some(current_time_s);

            if self.acc_time_s >= frame_time_s {
                // We subtracted the current frame from the accumulated time,
                // and there's still at least one full frame time left.
                //
                // This could mean that the game was paused, and we're coming
                // back with a huge delta time. Or that we're running too slow,
                // getting behind on frames.
                //
                // Either way, we don't want to burn the CPU by trying to catch
                // up.
                self.acc_time_s = 0.;
            }
        } else {
            // It's not time to run another frame yet.
            return false;
        }

        while self.process.can_step() {
            let Some(instructions) = &self.instructions else {
                return true;
            };

            self.process.step(instructions);

            if let Some(effect) = self.process.effects_mut().handle() {
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
                        self.process.effects_mut().trigger(new_effect);
                    }
                }
            }
        }

        true
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
