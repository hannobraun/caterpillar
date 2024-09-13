use capi_game_engine::command::Command;
use capi_process::Instructions;
use capi_protocol::{command::CommandExt, updates::Code, Versioned};
use gloo_net::http::{Request, Response};
use tokio::sync::watch;

use crate::{commands::CommandsToRuntimeTx, model::PersistentState};

pub type CodeRx = watch::Receiver<Instructions>;
pub type CodeTx = watch::Sender<Instructions>;

pub struct CodeFetcher {
    pub timestamp: u64,
}

impl CodeFetcher {
    pub async fn new(
        commands_to_runtime_tx: &CommandsToRuntimeTx,
        state: &mut PersistentState,
    ) -> anyhow::Result<Self> {
        let code = Request::get("/code").send().await;
        let timestamp =
            on_new_code(code, commands_to_runtime_tx, state).await?;

        Ok(Self { timestamp })
    }

    pub async fn wait_for_new_code(
        &mut self,
        commands_to_runtime_tx: &CommandsToRuntimeTx,
        state: &mut PersistentState,
    ) -> anyhow::Result<()> {
        let code = Request::get(&format!("/code/{}", self.timestamp))
            .send()
            .await;

        self.timestamp =
            on_new_code(code, commands_to_runtime_tx, state).await?;

        Ok(())
    }
}

async fn on_new_code(
    code: Result<Response, gloo_net::Error>,
    commands_to_runtime_tx: &CommandsToRuntimeTx,
    state: &mut PersistentState,
) -> anyhow::Result<u64> {
    let code = code?.text().await?;
    let code: Versioned<Code> = ron::from_str(&code)?;

    let instructions = state.on_new_code(code.inner);
    let command = Command::UpdateCode { instructions };
    commands_to_runtime_tx.send(command.serialize()).expect(
        "Command receiver lives in static variable, should never drop.",
    );

    Ok(code.timestamp)
}
