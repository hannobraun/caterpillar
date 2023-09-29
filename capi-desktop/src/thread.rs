use capi_core::{Interpreter, PlatformFunction, RuntimeState};
use crossbeam_channel::{Receiver, RecvError, TryRecvError};

use crate::{
    display::Display,
    platform::{self, Context, PixelOp},
};

pub fn run(code: &str, updates: Receiver<String>) -> anyhow::Result<()> {
    let (pixel_ops_tx, pixel_ops_rx) = crossbeam_channel::unbounded();

    let mut interpreter = Interpreter::new(code)?;
    let mut context = Context {
        pixel_ops: pixel_ops_tx,
    };
    let mut display = None;

    interpreter.register_platform([
        (
            "delay_ms",
            platform::delay_ms as PlatformFunction<platform::Context>,
        ),
        ("pixel_set", platform::pixel_set),
        ("print", platform::print),
    ]);

    loop {
        let runtime_state = interpreter.step(&mut context)?;

        for PixelOp::Set(position) in pixel_ops_rx.try_iter() {
            let mut d = display.map(Ok).unwrap_or_else(Display::new)?;

            d.set(position)?;

            display = Some(d);
        }

        let new_code = match runtime_state {
            RuntimeState::Running => match updates.try_recv() {
                Ok(new_code) => Some(new_code),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => break,
            },
            RuntimeState::Sleeping => {
                unreachable!(
                    "No desktop platform functions put runtime to sleep"
                )
            }
            RuntimeState::Finished => {
                eprintln!();
                eprintln!("> Program finished.");
                eprintln!("  > will restart on change to script");
                eprintln!("  > press CTRL-C to abort");
                eprintln!();

                match updates.recv() {
                    Ok(new_code) => Some(new_code),
                    Err(RecvError) => break,
                }
            }
        };

        if let Some(new_code) = new_code {
            interpreter.update(&new_code)?;
        }
    }

    Ok(())
}
