pub mod snake;

pub fn build(game: fn(&mut crate::syntax::Script)) -> crate::program::Process {
    let mut script = crate::syntax::Script::default();
    game(&mut script);
    crate::compiler::compile(script, "main")
}
