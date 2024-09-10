use capi_process::Instructions;
use capi_protocol::{updates::Code, Versioned};
use gloo_net::http::Response;
use tokio::sync::watch;

use crate::model::PersistentState;

pub type CodeRx = watch::Receiver<Instructions>;
pub type CodeTx = watch::Sender<Instructions>;

pub struct CodeManager {
    pub timestamp: u64,
}

pub async fn on_new_code(
    code: Result<Response, gloo_net::Error>,
    code_tx: &CodeTx,
    state: &mut PersistentState,
) -> u64 {
    let code = code.unwrap().text().await.unwrap();
    let code: Versioned<Code> = ron::from_str(&code).unwrap();

    code_tx
        .send(code.inner.instructions.clone())
        .expect("Code receiver should never drop.");

    state.code = Some(code.inner.clone());

    code.timestamp
}
