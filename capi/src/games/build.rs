pub fn build(game: fn(&mut crate::syntax::Script)) -> crate::process::Process {
    let mut script = crate::syntax::Script::default();
    game(&mut script);
    let (code, source_map) = crate::compiler::compile(&script);

    let entry = code.functions.get("main").cloned().unwrap();
    crate::process::Process::new(
        script.functions,
        source_map,
        code,
        entry,
        vec![crate::runtime::Value(crate::tiles::TILES_PER_AXIS as i8); 2],
    )
}
