use capi_compiler::{
    fragments::{self, Cluster, Fragments},
    source_map::SourceMap,
};
use capi_game_engine::host::GameEngineHost;
use capi_process::Process;

use super::Expression;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    pub name: String,
    pub body: Vec<Expression>,
}

impl Function {
    pub fn new(
        cluster: Cluster,
        function: fragments::Function,
        fragments: &Fragments,
        source_map: &SourceMap,
        process: &Process<GameEngineHost>,
    ) -> Self {
        let body = fragments
            .inner
            .iter_from(function.start)
            .cloned()
            .filter_map(|fragment| {
                Expression::new(fragment, fragments, source_map, process)
            })
            .collect();

        Self {
            name: cluster.name,
            body,
        }
    }
}
