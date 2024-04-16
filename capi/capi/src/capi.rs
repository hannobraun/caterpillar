use capi_runtime::{Program, Source};

pub fn program() -> Program {
    let mut source = Source::default();

    source.define("write_to_tile_buffer", |s| {
        s.w("last_tile_index")
            .w("first_tile_index")
            .w("set_all_tiles")
            .w("clean_up_arguments");
    });
    source.define("last_tile_index", |s| {
        s.w("mul").w("first_tile_index").w("add");
    });
    source.define("first_tile_index", |s| {
        s.v(256);
    });
    source.define("set_all_tiles", |s| {
        s.w("check_tile_index")
            .w("return_if_zero")
            .w("set_tile")
            .w("set_all_tiles");
    });
    source.define("check_tile_index", |s| {
        s.v(1).w("copy").v(1).w("copy").w("sub");
    });
    source.define("set_tile", |s| {
        s.v(1).w("store").w("inc_tile_index");
    });
    source.define("inc_tile_index", |s| {
        s.v(1).w("add");
    });
    source.define("clean_up_arguments", |s| {
        s.v(0).w("drop").v(0).w("drop");
    });

    source.compile("write_to_tile_buffer")
}
