use capi_runtime::Program;

pub fn program() -> Program {
    let mut program = Program::default();

    program.define("write_to_tile_buffer", |s| {
        s.w("last_tile_index")
            .w("first_tile_index")
            .w("set_all_tiles")
            .w("clean_up_arguments");
    });
    program.define("last_tile_index", |s| {
        s.w("mul").w("first_tile_index").w("add");
    });
    program.define("first_tile_index", |s| {
        s.v(256);
    });
    program.define("set_all_tiles", |s| {
        s.w("check_tile_index")
            .w("return_if_zero")
            .w("set_tile")
            .w("set_all_tiles");
    });
    program.define("check_tile_index", |s| {
        s.v(1).w("copy").v(1).w("copy").w("sub");
    });
    program.define("set_tile", |s| {
        s.v(1).w("store").w("inc_tile_index");
    });
    program.define("inc_tile_index", |s| {
        s.v(1).w("add");
    });
    program.define("clean_up_arguments", |s| {
        s.v(0).w("drop").v(0).w("drop");
    });

    let code = program.functions.clone().compile();
    let entry = code.symbols.resolve("write_to_tile_buffer");

    program.evaluator.update(code, entry);
    program.entry = entry;

    program
}
