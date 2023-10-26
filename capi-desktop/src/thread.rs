use std::thread;

use capi_core::{Interpreter, RuntimeState};
use crossbeam_channel::{Receiver, RecvError, Sender, TryRecvError};

use crate::platform::{self, Context, PixelOp};

pub struct DesktopThread {
    pub pixel_ops: Receiver<PixelOp>,
    lifeline: Sender<()>,
    join_handle: JoinHandle,
}

impl DesktopThread {
    pub fn run(
        code: String,
        updates: Receiver<String>,
    ) -> anyhow::Result<Self> {
        let (pixel_ops_tx, pixel_ops_rx) = crossbeam_channel::unbounded();
        let (lifeline_tx, lifeline_rx) = crossbeam_channel::bounded(0);

        let join_handle = thread::spawn(|| {
            run_inner(code, updates, lifeline_rx, pixel_ops_tx)
        });

        Ok(Self {
            pixel_ops: pixel_ops_rx,
            lifeline: lifeline_tx,
            join_handle,
        })
    }

    pub fn join(self) -> anyhow::Result<()> {
        join_inner(self.join_handle)
    }

    pub fn quit(self) -> anyhow::Result<()> {
        // This will signal the thread that it should stop.
        drop(self.lifeline);

        join_inner(self.join_handle)
    }
}

type JoinHandle = thread::JoinHandle<anyhow::Result<()>>;

fn run_inner(
    code: String,
    updates: Receiver<String>,
    lifeline: Receiver<()>,
    pixel_ops: Sender<PixelOp>,
) -> anyhow::Result<()> {
    let mut interpreter = Interpreter::new(&code)?;
    let mut context = Context {
        pixel_ops: platform::Sender { inner: pixel_ops },
    };

    platform::register(&mut interpreter);

    loop {
        if let Err(TryRecvError::Disconnected) = lifeline.try_recv() {
            // If the other end of the lifeline got dropped, that means we're
            // supposed to stop.
            break;
        }

        let runtime_state = interpreter.step(&mut context)?;

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

fn join_inner(join_handle: JoinHandle) -> anyhow::Result<()> {
    match join_handle.join() {
        Ok(result) => {
            // The result that the thread returned, which is possibly an
            // error.
            result
        }
        Err(err) => {
            // The thread panicked! Let's make sure this bubbles up to the
            // caller.
            std::panic::resume_unwind(err)
        }
    }
}
