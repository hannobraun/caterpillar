use tokio::sync::mpsc;

use crate::process::Process;

pub fn updates() -> (UpdatesTx, UpdatesRx) {
    let (tx, rx) = mpsc::unbounded_channel();

    let tx = UpdatesTx {
        inner: tx,
        process_at_client: None,
    };

    (tx, rx)
}

pub type UpdatesRx = mpsc::UnboundedReceiver<Update>;

pub enum Update {
    Process(Process),
}

#[derive(Clone)]
pub struct UpdatesTx {
    inner: mpsc::UnboundedSender<Update>,
    process_at_client: Option<Process>,
}

impl UpdatesTx {
    pub fn send_if_relevant(&mut self, update: Update) {
        let Update::Process(process) = update;

        if let Some(process_at_client) = &self.process_at_client {
            // The client has previously received a program. We don't want to
            // saturate the connection with useless updates, so use that to
            // determine, if we should send an update.

            if process_at_client.can_step() && process.can_step() {
                // While the program is running, sending updates on every change
                // would result in too many updates.
                //
                // Let's check if there's a change that we consider worthy of
                // sending an update for.

                let breakpoints_unchanged =
                    process_at_client.breakpoints == process.breakpoints;

                if breakpoints_unchanged {
                    return;
                }
            }
        }
        if self.process_at_client.as_ref() == Some(&process) {
            // Client already has this program. Don't need to send it again.
            return;
        }

        self.process_at_client = Some(process.clone());
        self.inner.send(Update::Process(process.clone())).unwrap();
    }
}
