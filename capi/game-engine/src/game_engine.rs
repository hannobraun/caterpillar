use std::collections::VecDeque;

use capi_process::{Effect, Instructions, Process, Value};

use crate::{
    command::Command,
    display::{self, TILES_PER_AXIS},
    host::GameEngineEffect,
    memory::Memory,
};

#[derive(Debug)]
pub struct GameEngine {
    pub process: Process,

    arguments: [Value; 2],
    last_frame_start_s: Option<f64>,
    instructions: Option<Instructions>,
    memory: Memory,
    input: VecDeque<u8>,
    random: VecDeque<i32>,
}

impl GameEngine {
    pub fn new() -> Self {
        let arguments = [Value::from(TILES_PER_AXIS); 2];

        Self {
            process: Process::new(arguments),
            arguments,
            last_frame_start_s: None,
            instructions: None,
            memory: Memory::default(),
            input: VecDeque::new(),
            random: VecDeque::new(),
        }
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn on_input(&mut self, value: u8) {
        self.input.push_back(value);
    }

    pub fn on_command(&mut self, command: Command) {
        if let Command::Reset = command {
            self.memory = Memory::default();
        }

        match command {
            Command::UpdateCode { instructions } => {
                self.instructions = Some(instructions);
            }
            Command::ClearBreakpointAndEvaluateNextInstruction => {
                if let Some(Effect::Breakpoint) =
                    self.process.effects().inspect_first()
                {
                    self.process.effects_mut().handle_first();
                } else {
                    // This shouldn't happen, unless there's a bug in the
                    // debugger. There's no point in panicking here though.
                    //
                    // We should signal this back to the debugger, so it can
                    // display a prominent message to the user. But right now,
                    // we don't have a good way to do so.
                }

                if let Some(instructions) = &self.instructions {
                    self.process.evaluate_next_instruction(instructions);
                } else {
                    // Same as above: This should only happen if the debugger is
                    // buggy.
                }
            }
            Command::Continue => {
                if let Some(Effect::Breakpoint) =
                    self.process.effects_mut().inspect_first()
                {
                    self.process.effects_mut().handle_first();
                }
            }
            Command::IgnoreNextInstruction => {
                self.process.ignore_next_instruction();
            }
            Command::Reset => {
                self.process.reset(self.arguments);
            }
            Command::Step => {
                if let Some(Effect::Breakpoint) =
                    self.process.effects_mut().inspect_first()
                {
                    if let Some(Effect::Breakpoint) =
                        self.process.effects().inspect_first()
                    {
                        let and_stop_at =
                            self.process.evaluator().next_instruction;
                        self.process
                            .breakpoints_mut()
                            .set_ephemeral(and_stop_at);

                        self.process.effects_mut().handle_first();
                    }
                } else {
                    // If we're not stopped at a breakpoint, we can't step.
                    // It would be better, if this resulted in an explicit
                    // error that is sent to the debugger, instead of
                    // silently being ignored here.
                }
            }
            Command::Stop => {
                self.process.effects_mut().trigger(Effect::Breakpoint);
            }
        }
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

    pub fn run_until_end_of_frame(
        &mut self,
        current_time_s: f64,
        pixels: &mut [u8],
    ) -> bool {
        // For now, we're targeting an unambitious 30 fps.
        let frame_time_s = 1. / 30.;

        if let Some(last_frame_start_s) = self.last_frame_start_s {
            let time_since_last_frame_start_s =
                current_time_s - last_frame_start_s;

            if time_since_last_frame_start_s >= frame_time_s * 2. {
                // It's time for another frame, but it seems that has been true
                // for a while. This could mean that the game was paused, or
                // that we're running too slow, getting behind on frames.
                //
                // Either way, we don't want to burn the CPU, trying to catch
                // up.
                self.last_frame_start_s = Some(current_time_s);
            } else if time_since_last_frame_start_s >= frame_time_s {
                // It's time for another frame, and we don't seem to be getting
                // behind.
                //
                // In this case, don't remember the current time as the start
                // time of the frame, but instead just advance that by the
                // nominal frame time. This way, any timing inaccuracies in
                // calling this function should get smoothed out a bit, on
                // average.
                self.last_frame_start_s =
                    Some(last_frame_start_s + frame_time_s);
            } else {
                // It's not time for another frame yet!
                return false;
            }
        } else {
            // This seems to be the first frame. Just run it immediately.
            self.last_frame_start_s = Some(current_time_s);
        }

        while self.process.state().is_running() {
            let Some(instructions) = &self.instructions else {
                return true;
            };

            self.process.evaluate_next_instruction(instructions);

            if let Some(effect) = self.process.effects_mut().handle_first() {
                match self.handle_effect(&effect, pixels) {
                    Ok(EffectOutcome::Handled) => {
                        self.process.ignore_next_instruction();
                    }
                    Ok(EffectOutcome::WasSubmit) => {
                        self.process.ignore_next_instruction();

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
            GameEngineEffect::Halt => {
                return Ok(EffectOutcome::Unhandled);
            }
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
