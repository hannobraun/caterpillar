use crossbeam_channel::Sender;

pub struct ScriptLoader {
    pub sender: Sender<anyhow::Result<String>>,
}

impl ScriptLoader {
    pub fn new(sender: Sender<anyhow::Result<String>>) -> Self {
        Self { sender }
    }
}
