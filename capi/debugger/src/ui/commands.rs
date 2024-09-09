use super::{Action, ActionsTx};

pub async fn send_command(action: Action, actions: ActionsTx) {
    if let Err(err) = actions.send(action) {
        log::error!("Error sending command: {err}");
    }
}
