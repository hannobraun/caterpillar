use capi_runtime::{Program, Source};

pub fn program() -> Program {
    let mut source = Source::default();

    source.define("main", |s| {
        s.w("store_tile_field_size")
            .w("init_frame_count")
            .w("init_tile_value")
            .w("main_inner");
    });
    source.define("main_inner", |s| {
        s.w("update_tile_value")
            .w("draw")
            .w("count_frame")
            .w("main_inner");
    });
    source.define("update_tile_value", |s| {
        s
            .c("Get a copy of the current frame count.")
            .w("frame_count")
            .w("load")
            .c("We want to make updates at regular intervals. Determine, if")
            .c("this frame is one we need to make an update in. If not, we're")
            .c("done.")
            .v(120)
            .w("remainder")
            .w("return_if_non_zero")
            .c("This is the right frame. Make a copy of the current one, then")
            .c("speculatively replace is with `1`.")
            .w("tile_value")
            .w("load")
            .v(1)
            .w("tile_value")
            .w("store")
            .c("If the current tile value is `0`, the `1` we placed is correct")
            .c("and we are done.")
            .w("return_if_zero")
            .c("The current tile value is `1`. That means we need to replace")
            .c("the `1` we speculatively placed with a `0`.")
            .v(0)
            .w("tile_value")
            .w("store");
    });
    source.define("draw", |s| {
        s.w("clear_all_tiles")
            .w("draw_snake")
            .c("Wait until the display system is ready to process the next")
            .c("frame.")
            .w("submit_frame");
    });
    source.define("clear_all_tiles", |s| {
        s.v(0).w("write_all_tiles");
    });
    source.define("draw_snake", |s| {
        s.v(15).v(15).v(1).w("write_tile");
    });
    source.define("write_all_tiles", |s| {
        s.c("`write_all_tiles_inner` needs a tile position to count up.")
            .c("Initialize it with the position of the first tile.")
            .w("first_tile_position")
            .c("Arguments are in place. We're ready to set all tiles.")
            .w("write_all_tiles_inner")
            .w("drop_position")
            .w("drop_tile_value");
    });
    source.define("first_tile_position", |s| {
        s.v(0).v(0);
    });
    source.define("write_all_tiles_inner", |s| {
        s.c("This is a recursive function, so we might have been at it for a")
            .c("while, if we make it here. Check if the current tile position")
            .c("has reached the last one, which would let us know we're done.")
            .w("check_tile_position")
            .c("Return, if current position has reached beyond the last tile.")
            .w("return_if_zero")
            .c("Put the tile value we're supposed to write to the top of the")
            .c("stack, then write it.")
            .v(2)
            .w("copy")
            .w("write_tile")
            .w("increment_tile_position")
            .w("write_all_tiles_inner");
    });
    source.define("check_tile_position", |s| {
        s.c("Copy height of tile field.")
            .w("tile_field_height")
            .w("load")
            .c("Copy y-coordinate of current position.")
            .v(1)
            .w("copy")
            .c("Leave zero, if the y-coordinate has advanced beyond the last")
            .c("line of the tile field. Otherwise, leave non-zero value.")
            .w("sub");
    });
    source.define("increment_tile_position", |s| {
        s.c("Copy the width of the tile field.")
            .w("tile_field_width")
            .w("load")
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
    source.define("drop_position", |s| {
        s.v(0).w("drop").v(0).w("drop");
    });
    source.define("drop_tile_value", |s| {
        s.v(0).w("drop");
    });
    source.define("count_frame", |s| {
        s.c("We only have 8 bits to count, so we need to reset the count")
            .c("every so often. To keep things predictable, let's reset only")
            .c("at full seconds. Assuming 60 frames per second, `240` is the")
            .c("highest number we can count up to.")
            .c("")
            .c("Since we start counting at `1`, we need to reset *after* we")
            .c("reach a count of `240`, or we won't reset on a full second.")
            .c("Let's prepare the number to compare to for later use.")
            .v(241)
            .c("Grab the current frame count.")
            .w("frame_count")
            .w("load")
            .c("Increment the frame count.")
            .v(1)
            .w("add")
            .c("Place a copy of the new new frame count back where it came")
            .c("from.")
            .v(0)
            .w("copy")
            .w("frame_count")
            .w("store")
            .c("We have a copy of the new frame count left on the top of the")
            .c("stack. Let's see if we counted up to the maximum value. If")
            .c("not, we're done.")
            .w("sub")
            .w("return_if_non_zero")
            .c("We have counted up to the maximum value. Reset the frame")
            .c("count.")
            .w("init_frame_count");
    });
    source.define("store_tile_field_size", |s| {
        s.w("tile_field_height")
            .w("store")
            .w("tile_field_width")
            .w("store");
    });
    source.define("tile_field_width", |s| {
        s.c("Address of the tile field width in memory.").v(0);
    });
    source.define("tile_field_height", |s| {
        s.c("Address of the tile field height in memory.").v(1);
    });
    source.define("init_frame_count", |s| {
        s.v(1).w("frame_count").w("store");
    });
    source.define("frame_count", |s| {
        s.c("Address of the frame count in memory.").v(2);
    });
    source.define("init_tile_value", |s| {
        s.v(1).w("tile_value").w("store");
    });
    source.define("tile_value", |s| {
        s.c("Address of the tile value in memory.").v(3);
    });

    source.compile("main")
}
