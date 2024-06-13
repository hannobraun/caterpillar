use crate::{
    compiler::compile, process::Process, runtime::Value, syntax::Script,
    tiles::TILES_PER_AXIS,
};

pub fn build(game: fn(&mut Script)) -> Process {
    let mut script = crate::syntax::Script::default();
    game(&mut script);
    let (code, source_map) = compile(&script);

    let entry = code.functions.get("main").cloned().unwrap();
    Process::new(
        script.functions,
        source_map,
        code,
        entry,
        vec![Value(TILES_PER_AXIS as i8); 2],
    )
}
