use std::panic;

use capi_game_engine::game_engine::GameEngine;
use capi_process::{CoreEffect, Effect};
use capi_protocol::{
    command::{Command, SerializedCommand},
    updates::Updates,
};

use crate::ffi_out::on_panic;

pub struct RuntimeState {
    pub game_engine: GameEngine,
    pub commands: Vec<SerializedCommand>,
    pub updates: Updates,
}

impl RuntimeState {
    pub fn new() -> Self {
        panic::set_hook(Box::new(|panic_info| {
            on_panic(&panic_info.to_string());
        }));

        Self {
            game_engine: GameEngine::new(),
            commands: Vec::new(),
            updates: Updates::default(),
        }
    }

    pub fn update(&mut self, pixels: &mut [u8]) {
        for command in self.commands.drain(..) {
            let command = Command::deserialize(command);

            match command {
                Command::BreakpointClear { instruction } => {
                    self.game_engine
                        .process
                        .clear_durable_breakpoint(&instruction);
                }
                Command::BreakpointSet { instruction } => {
                    self.game_engine
                        .process
                        .set_durable_breakpoint(instruction);
                }
                Command::Continue { and_stop_at } => {
                    self.game_engine.process.continue_(and_stop_at);
                }
                Command::Reset => self.game_engine.reset(),
                Command::Step => {
                    if let Some(Effect::Core(CoreEffect::Breakpoint)) = self
                        .game_engine
                        .process
                        .state()
                        .first_unhandled_effect()
                    {
                        let and_stop_at =
                            self.game_engine.process.stack().next_instruction();
                        self.game_engine.process.continue_(Some(and_stop_at))
                    } else {
                        // If we're not stopped at a breakpoint, we can't step.
                        // It would be better, if this resulted in an explicit
                        // error that is sent to the debugger, instead of
                        // silently being ignored here.
                    }
                }
                Command::Stop => {
                    self.game_engine.process.stop();
                }
            }
        }

        self.game_engine.run_until_end_of_frame(pixels);

        self.updates.queue_updates(
            &self.game_engine.process,
            self.game_engine.memory(),
        );
    }
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self::new()
    }
}
