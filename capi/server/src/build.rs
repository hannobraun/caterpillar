use std::str;

use capi_compiler::compiler::compile;
use capi_process::Bytecode;
use capi_protocol::update::SourceCode;
use tokio::process::Command;

pub async fn build_snake() -> anyhow::Result<(SourceCode, Bytecode)> {
    let script = Command::new("cargo")
        .arg("run")
        .args(["--package", "snake"])
        .output()
        .await?
        .stdout;
    let script = str::from_utf8(&script).unwrap();
    let script = ron::from_str(script).unwrap();

    let (bytecode, source_map) = compile(&script);
    let source_code = SourceCode {
        functions: script.functions,
        source_map,
    };

    Ok((source_code, bytecode))
}
