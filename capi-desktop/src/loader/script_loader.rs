use crossbeam_channel::Sender;

pub struct ScriptLoader {
    pub sender: Sender<anyhow::Result<String>>,
}
