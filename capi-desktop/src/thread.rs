use std::{path::PathBuf, thread};

use capi_core::RuntimeState;
use crossbeam_channel::{Receiver, RecvError, Sender, TryRecvError};

use crate::{
    loader::Loader,
    platform::{self, PixelOp, PlatformContext},
    Interpreter,
};

pub struct DesktopThread {
    pub pixel_ops: Receiver<PixelOp>,
    lifeline: Sender<()>,
    join_handle: JoinHandle,
}

impl DesktopThread {
    pub fn run(entry_script_path: PathBuf) -> anyhow::Result<Self> {
        struct RunProgram;

        impl RunTarget for RunProgram {
            fn step(
                &self,
                interpreter: &mut Interpreter,
                platform_context: &mut PlatformContext,
            ) -> anyhow::Result<RuntimeState> {
                let runtime_state = interpreter.step(platform_context)?;
                Ok(runtime_state)
            }

            fn finish(&self) {
                eprintln!();
                eprintln!("> Program finished.");
                eprintln!("  > will restart on change to script");
                eprintln!("  > press CTRL-C to abort");
                eprintln!();
            }
        }

        Self::new(entry_script_path, RunProgram)
    }

    pub fn test(entry_script_path: PathBuf) -> anyhow::Result<Self> {
        struct RunTests;

        impl RunTarget for RunTests {
            fn step(
                &self,
                interpreter: &mut Interpreter,
                platform_context: &mut PlatformContext,
            ) -> anyhow::Result<RuntimeState> {
                interpreter.run_tests(platform_context)?;
                Ok(RuntimeState::Finished)
            }

            fn finish(&self) {
                eprintln!();
                eprintln!("> Test run finished.");
                eprintln!("  > will re-run on change to script");
                eprintln!("  > press CTRL-C to abort");
                eprintln!();
            }
        }

        Self::new(entry_script_path, RunTests)
    }

    fn new(
        entry_script_path: PathBuf,
        run_target: impl RunTarget,
    ) -> anyhow::Result<Self> {
        let (pixel_ops_tx, pixel_ops_rx) = crossbeam_channel::unbounded();
        let (lifeline_tx, lifeline_rx) = crossbeam_channel::bounded(0);

        let join_handle = thread::spawn(|| {
            Self::run_inner(
                entry_script_path,
                lifeline_rx,
                pixel_ops_tx,
                run_target,
            )
        });

        Ok(Self {
            pixel_ops: pixel_ops_rx,
            lifeline: lifeline_tx,
            join_handle,
        })
    }

    fn run_inner(
        entry_script_path: PathBuf,
        lifeline: Receiver<()>,
        pixel_ops: Sender<PixelOp>,
        run_target: impl RunTarget,
    ) -> anyhow::Result<()> {
        let parent = None;

        let mut loader = Loader::new();
        let code = loader.load(&entry_script_path, parent)?;

        let mut interpreter = Interpreter::new()?;
        interpreter.update(&code, parent)?;
        let mut platform_context =
            PlatformContext::new(entry_script_path, loader)
                .with_pixel_ops_sender(pixel_ops);

        platform::register(&mut interpreter);

        loop {
            if let Err(TryRecvError::Disconnected) = lifeline.try_recv() {
                // If the other end of the lifeline got dropped, that means
                // we're supposed to stop.
                break;
            }

            match platform_context.loader.updates().try_recv() {
                Ok(new_code) => {
                    let (parent, new_code) = new_code?;
                    interpreter.update(&new_code, parent)?;
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => break,
            }

            let runtime_state =
                run_target.step(&mut interpreter, &mut platform_context)?;

            match runtime_state {
                RuntimeState::Running => {}
                RuntimeState::Sleeping => {
                    unreachable!(
                        "No desktop platform functions put runtime to sleep"
                    )
                }
                RuntimeState::Finished => {
                    run_target.finish();

                    match platform_context.loader.updates().recv() {
                        Ok(new_code) => {
                            let (parent, new_code) = new_code?;
                            interpreter.update(&new_code, parent)?;
                        }
                        Err(RecvError) => break,
                    }
                }
            };
        }

        Ok(())
    }

    pub fn join(self) -> anyhow::Result<()> {
        Self::join_inner(self.join_handle)
    }

    pub fn quit(self) -> anyhow::Result<()> {
        // This will signal the thread that it should stop.
        drop(self.lifeline);

        Self::join_inner(self.join_handle)
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
}

type JoinHandle = thread::JoinHandle<anyhow::Result<()>>;

trait RunTarget: Send + 'static {
    fn step(
        &self,
        interpreter: &mut Interpreter,
        platform_context: &mut PlatformContext,
    ) -> anyhow::Result<RuntimeState>;

    fn finish(&self);
}
