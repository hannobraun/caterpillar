use super::{Action, ActionsTx};

pub async fn send_command(action: Action, actions: ActionsTx) {
    if let Err(err) = actions.send(action) {
        log::error!(
            "Sending a UI action failed, as the receive is no longer \
            available: {err:#?}\n\
            \n\
            This is most likely a bug in the Caterpillar debugger."
        );
    }
}
