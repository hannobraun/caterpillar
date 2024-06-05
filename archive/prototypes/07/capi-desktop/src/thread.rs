use std::{path::PathBuf, thread};

use anyhow::Context;
use capi_core::runtime::{
    evaluator::RuntimeState, interpreter::Interpreter, test_runner::run_tests,
};
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use crossterm::style::Stylize;
use error_reporter::Report;

use crate::{
    loader::Loader,
    platform::{DesktopPlatform, PixelOp, PlatformContext},
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
            fn finish(
                &self,
                _: &mut Interpreter<DesktopPlatform>,
                _: PlatformContext,
            ) -> anyhow::Result<()> {
                eprintln!();
                eprintln!("> Program finished.");
                eprintln!("  > will restart on change to script");
                eprintln!("  > press CTRL-C to abort");
                eprintln!();

                Ok(())
            }
        }

        Self::new(entry_script_path, RunProgram)
    }

    pub fn test(entry_script_path: PathBuf) -> anyhow::Result<Self> {
        struct RunTests;

        impl RunTarget for RunTests {
            fn finish(
                &self,
                interpreter: &mut Interpreter<DesktopPlatform>,
                platform_context: PlatformContext,
            ) -> anyhow::Result<()> {
                let test_report = run_tests(interpreter, platform_context)?;

                for report in test_report.inner {
                    if report.result.is_ok() {
                        print!("{}", "PASS".bold().green());
                    } else {
                        print!("{}", "FAIL".bold().red());
                    }

                    println!("  {}", report.test_name);

                    if let Err(err) = report.result {
                        println!("{}", Report::new(err).pretty(true));
                        println!();
                    }
                }

                eprintln!();
                eprintln!("> Test run finished.");
                eprintln!("  > will re-run on change to script");
                eprintln!("  > press CTRL-C to abort");
                eprintln!();

                Ok(())
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
            Self::thread(
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

    fn thread(
        entry_script_path: PathBuf,
        lifeline: Receiver<()>,
        pixel_ops: Sender<PixelOp>,
        run_target: impl RunTarget,
    ) -> anyhow::Result<()> {
        let (mut loader, scripts) = Loader::new(entry_script_path)
            .context("Failed to initialize loader")?;
        let mut interpreter = Interpreter::new(scripts)
            .context("Failed to create interpreter")?;

        loop {
            if let Err(TryRecvError::Disconnected) = lifeline.try_recv() {
                // If the other end of the lifeline got dropped, that means
                // we're supposed to stop.
                break;
            }

            if loader
                .apply_update_if_available(interpreter.scripts())
                .context("Error while checking for updated scripts")?
            {
                interpreter
                    .update()
                    .context("Failed to update scripts while running")?;
            }

            let runtime_state = interpreter
                .step(&mut PlatformContext::new(&pixel_ops))
                .context("Error while stepping interpreter")?;

            match runtime_state {
                RuntimeState::Running => {}
                RuntimeState::Sleeping => {
                    // None of the desktop platform functions put the runtime to
                    // sleep.
                    unreachable!()
                }
                RuntimeState::Finished => {
                    run_target.finish(
                        &mut interpreter,
                        PlatformContext::new(&pixel_ops),
                    )?;

                    let updates = loader
                        .wait_for_updates()
                        .context("Error waiting for updates")?;

                    for (path, code) in updates {
                        interpreter.scripts().inner.insert(path, code);
                    }

                    interpreter
                        .update()
                        .context("Failed to update scripts while finished")?;
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
    fn finish(
        &self,
        interpreter: &mut Interpreter<DesktopPlatform>,
        platform_context: PlatformContext,
    ) -> anyhow::Result<()>;
}
