use tokio::sync::watch;

use crate::program::Process;

pub fn updates(program: &Process) -> (UpdatesTx, UpdatesRx) {
    let (tx, rx) = watch::channel(program.clone());

    let tx = UpdatesTx {
        inner: tx,
        program_at_client: None,
    };

    (tx, rx)
}

pub type UpdatesRx = watch::Receiver<Process>;

#[derive(Clone)]
pub struct UpdatesTx {
    inner: UpdatesTxInner,
    program_at_client: Option<Process>,
}

impl UpdatesTx {
    pub fn send_if_relevant_change(&mut self, program: &Process) {
        if let Some(program_at_client) = &self.program_at_client {
            // The client has previously received a program. We don't want to
            // saturate the connection with useless updates, so use that to
            // determine, if we should send an update.

            if program_at_client.can_step() && program.can_step() {
                // While the program is running, sending updates on every change
                // would result in too many updates.
                //
                // Let's check if there's a change that we consider worthy of
                // sending an update for.

                let breakpoints_unchanged =
                    program_at_client.breakpoints == program.breakpoints;

                if breakpoints_unchanged {
                    return;
                }
            }
        }
        if self.program_at_client.as_ref() == Some(program) {
            // Client already has this program. Don't need to send it again.
            return;
        }

        self.program_at_client = Some(program.clone());
        self.inner.send(program.clone()).unwrap();
    }
}

pub type UpdatesTxInner = watch::Sender<Process>;
