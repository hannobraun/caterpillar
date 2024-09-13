use capi_process::Instructions;
use capi_protocol::{updates::Code, Versioned};
use gloo_net::http::{Request, Response};
use tokio::sync::watch;

use crate::model::PersistentState;

pub type CodeRx = watch::Receiver<Instructions>;
pub type CodeTx = watch::Sender<Instructions>;

pub struct CodeManager {
    pub timestamp: u64,
}

impl CodeManager {
    pub async fn new(
        code_tx: &CodeTx,
        state: &mut PersistentState,
    ) -> anyhow::Result<Self> {
        let code = Request::get("/code").send().await;
        let timestamp = on_new_code(code, code_tx, state).await?;

        Ok(Self { timestamp })
    }

    pub async fn wait_for_new_code(
        &mut self,
        code_tx: &CodeTx,
        state: &mut PersistentState,
    ) -> anyhow::Result<()> {
        let code = Request::get(&format!("/code/{}", self.timestamp))
            .send()
            .await;

        self.timestamp = on_new_code(code, code_tx, state).await?;

        Ok(())
    }
}

async fn on_new_code(
    code: Result<Response, gloo_net::Error>,
    code_tx: &CodeTx,
    state: &mut PersistentState,
) -> anyhow::Result<u64> {
    let code = code?.text().await?;
    let code: Versioned<Code> = ron::from_str(&code)?;

    state.on_new_code(code.inner.clone());

    code_tx
        .send(code.inner.instructions.clone())
        .expect("Code receiver should never drop.");

    Ok(code.timestamp)
}
