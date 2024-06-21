use std::collections::VecDeque;

use rand::random;
use tokio::sync::mpsc::{self, error::TryRecvError};

use crate::{
    compiler::compile,
    debugger::{
        model::DebugCommand,
        ui::{self, CommandsRx},
    },
    display::Display,
    ffi,
    games::snake::snake,
    process::Process,
    runtime::{self, BuiltinEffect, Code, EvaluatorEffect, Value},
    tiles::{NUM_TILES, TILES_PER_AXIS},
    updates::{Updates, UpdatesTx},
};

pub struct RuntimeState {
    pub code: Code,
    pub entry: runtime::Function,
    pub arguments: Vec<Value>,
    pub process: Process,
    pub memory: Memory,
    pub input: Input,
    pub tiles: [u8; NUM_TILES],
    pub display: Option<Display>,
    pub commands_rx: CommandsRx,
    pub updates_tx: UpdatesTx,
    pub updates: Updates,
}

impl RuntimeState {
    pub fn new() -> Self {
        let mut script = crate::syntax::Script::default();
        snake(&mut script);

        let (code, source_map) = compile(&script);

        let entry = code.functions.get("main").cloned().unwrap();
        let arguments = vec![Value(TILES_PER_AXIS as i8); 2];

        let mut process = Process::default();
        process.reset(entry.clone(), arguments.clone());

        let memory = Memory::default();

        let input = Input::default();
        let (commands_tx, commands_rx) = mpsc::unbounded_channel();

        let (updates_tx, updates_rx) = mpsc::unbounded_channel();
        let mut updates = Updates::new();

        updates.queue_source_code(script.functions, source_map);
        ui::start(updates_rx, commands_tx);

        // While we're still using `pixels`, the `Display` constructor needs to
        // be async. We need to do some acrobatics here to deal with that.
        leptos::spawn_local(async {
            let display = Display::new().await.unwrap();

            let mut state = ffi::STATE.lock().unwrap();
            let state = state.get_or_insert_with(Default::default);

            state.display = Some(display);
        });

        Self {
            code,
            entry,
            arguments,
            process,
            memory,
            input,
            tiles: [0; NUM_TILES],
            display: None,
            commands_rx,
            updates_tx,
            updates,
        }
    }

    pub fn update(&mut self) {
        let Some(display) = self.display.as_mut() else {
            // Display not initialized yet.
            return;
        };

        loop {
            match self.commands_rx.try_recv() {
                Ok(command) => match command {
                    DebugCommand::BreakpointClear { location } => {
                        self.process.clear_durable_breakpoint(location);
                    }
                    DebugCommand::BreakpointSet { location } => {
                        self.process.set_durable_breakpoint(location);
                    }
                    DebugCommand::Continue { and_stop_at } => {
                        self.process.continue_(and_stop_at);
                    }
                    DebugCommand::Reset => {
                        self.process
                            .reset(self.entry.clone(), self.arguments.clone());
                        self.memory.zero();
                    }
                    DebugCommand::Step => {
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
                    DebugCommand::Stop => {
                        self.process.stop();
                    }
                },
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(TryRecvError::Disconnected) => {
                    // The other end has hung up, which happens during
                    // shutdown. Shut down this task, too.
                    return;
                }
            }
        }

        while self.process.state().can_step() {
            self.process.step(&self.code);

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
                    BuiltinEffect::SetTile { x, y, value } => {
                        let x = *x;
                        let y = *y;
                        let value = *value;

                        self.process.handle_first_effect();

                        display.set_tile(
                            x.into(),
                            y.into(),
                            value,
                            &mut self.tiles,
                        );
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
                        self.process.push([Value(random())]);
                        self.process.handle_first_effect();
                    }
                }
            }
        }

        self.updates.queue_updates(&self.process, &self.memory);

        display.render(&self.tiles);
    }
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Memory {
    #[serde(with = "serde_big_array::BigArray")]
    pub inner: [Value; 256],
}

impl Memory {
    pub fn zero(&mut self) {
        *self = Self::default();
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            inner: [Value(0); 256],
        }
    }
}

#[derive(Default)]
pub struct Input {
    pub buffer: VecDeque<u8>,
}
