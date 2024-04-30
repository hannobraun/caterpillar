use capi_runtime::{Program, Source};

pub fn program() -> Program {
    let mut source = Source::default();

    source.define("write_to_tile_buffer", |s| {
        s
            .c("We have the size of the tile field already on the stack.")
            .c("This will be used by `set_all_tiles` to traverse positions,")
            .c("and to determine once it's finished.")
            .c("")
            .c("In addition, it also needs the position of the first tile,")
            .c("from which it will count up.")
            .w("first_tile_position")
            .c("Arguments are in place. We're ready to set all tiles.")
            .w("set_all_tiles")
            .c("`set_all_tiles` leaves its arguments on the stack, except that")
            .c("it has counted up the first tile position, which should now be")
            .c("identical to the last one.")
            .w("clean_up_arguments");
    });
    source.define("first_tile_position", |s| {
        s.v(0).v(0);
    });
    source.define("set_all_tiles", |s| {
        s.c("This is a recursive function, so we might have been at it for a")
            .c("while, if we make it here. Check if the current tile position")
            .c("has reached the last one, which would let us know we're done.")
            .w("check_tile_position")
            .c("Return, if current position has reached beyond the last tile.")
            .w("return_if_zero")
            .w("set_tile")
            .w("increment_tile_position")
            .w("set_all_tiles");
    });
    source.define("check_tile_position", |s| {
        s.c("Copy height of tile field.")
            .v(2)
            .w("copy")
            .c("Copy y-coordinate of current position.")
            .v(1)
            .w("copy")
            .c("Leave zero, if the y-coordinate has advanced beyond the last")
            .c("line of the tile field. Otherwise, leave non-zero value.")
            .w("sub");
    });
    source.define("set_tile", |s| {
        s.v(1).w("tile");
    });
    source.define("increment_tile_position", |s| {
        s.c("Copy the width of the tile field.")
            .v(3)
            .w("copy")
            .c("Copy the x-coordinate of the current position.")
            .v(2)
            .w("copy")
            .c("Increment the x-coordinate.")
            .v(1)
            .w("add")
            .c("Remove the old x-coordinate to make space for the updated one.")
            .v(3)
            .w("drop")
            .c("Make a copy of the updated x-coordinate.")
            .v(0)
            .w("copy")
            .c("Put the updated x-coordinate where the old one was.")
            .v(3)
            .w("place")
            .c("Leave zero, if the x-coordinate has advanced beyond the width.")
            .w("sub")
            .c("Unless the x-coordinate has advanced beyond the width, we're")
            .c("done here.")
            .w("return_if_non_zero")
            .c("Remove the overflowed x-coordinate from the stack.")
            .v(1)
            .w("drop")
            .c("Reset the x-coordinate back to zero.")
            .v(0)
            .v(1)
            .w("place")
            .c("Increment y-coordinate.")
            .v(1)
            .w("add");
    });
    source.define("clean_up_arguments", |s| {
        s.v(0)
            .w("drop")
            .v(0)
            .w("drop")
            .v(0)
            .w("drop")
            .v(0)
            .w("drop");
    });

    source.compile("write_to_tile_buffer")
}
