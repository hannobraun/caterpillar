use capi_runtime::{Program, Source};

pub fn program() -> Program {
    let mut source = Source::default();

    // Main loop
    source.define("main", |s| {
        s.w("tile_field_size")
            .w("store_vector")
            .w("init")
            .w("main_inner");
    });
    source.define("main_inner", |s| {
        s.w("draw").w("update").w("main_inner");
    });

    // Draw
    source.define("draw", |s| {
        s.w("clear_all_tiles")
            .w("draw_snake")
            .c("This blocks until the display system is ready to process the")
            .c("next frame.")
            .w("submit_frame");
    });
    source.define("clear_all_tiles", |s| {
        s.v(0).w("write_all_tiles");
    });
    source.define("draw_snake", |s| {
        s.v(0)
            .w("position")
            .w("vec_load")
            .v(1)
            .w("write_tile")
            .w("drop_vector");
    });

    // Draw - write tiles
    source.define("write_all_tiles", |s| {
        s.b(["tile_value"])
            .w("init_tile_index")
            .w("tile_value")
            .w("write_all_tiles_inner")
            .w("drop_vector");
    });
    source.define("write_all_tiles_inner", |s| {
        s.b(["tile_value"])
            .c("This is a recursive function, so we might have been at it for")
            .c("a while, if we make it here. Check if the tile index has gone")
            .c("beyond the last tile, which would let us know that we're done.")
            .v(0)
            .w("copy")
            .w("check_tile_index")
            .w("return_if_zero")
            .c("Apparently we're not done yet.")
            .w("tile_value")
            .w("write_tile")
            .w("increment_tile_index")
            .w("tile_value")
            .w("write_all_tiles_inner");
    });

    // Draw - write tiles - tile index
    source.define("init_tile_index", |s| {
        s.v(0).v(0);
    });
    source.define("check_tile_index", |s| {
        s.b(["tile_y"])
            .w("tile_field_size")
            .w("y")
            .w("load")
            .w("tile_y")
            .c("Leave zero, if the y-coordinate has advanced beyond the last")
            .c("line of the tile field. Otherwise, leave non-zero value.")
            .w("sub");
    });
    source.define("increment_tile_index", |s| {
        s.b(["tile_x", "tile_y"])
            .c("Increment the x-coordinate.")
            .w("tile_x")
            .v(1)
            .w("add")
            .b(["tile_x_new"])
            .c("Check if the x coordinate has advanced beyond the width.")
            .w("tile_field_size")
            .w("x")
            .w("load")
            .w("tile_x_new")
            .w("sub")
            .b(["zero_if_x_overflowed"])
            .c("Unless the x-coordinate has advanced beyond the width, we're")
            .c("done here.")
            .w("tile_x_new")
            .w("tile_y")
            .w("zero_if_x_overflowed")
            .w("return_if_non_zero")
            .c("Looks like we're not done!")
            .b(["tile_x_new", "tile_y"])
            .c("Increment y-coordinate.")
            .w("tile_y")
            .v(1)
            .w("add")
            .b(["tile_y_new"])
            .c("Return updated coordinates")
            .v(0)
            .w("tile_y_new");
    });

    // Game state
    source.define("init", |s| {
        s.w("init_frame_count")
            .w("init_should_game_run")
            .w("init_positions")
            .w("init_velocity")
            .w("init_next_position");
    });
    source.define("update", |s| {
        s.w("update_frame_count")
            .c("Update we want to do every frame are done. Get a copy of the")
            .c("current frame count, to figure out if we need to do more.")
            .w("frame_count")
            .w("load")
            .c("We want to make updates at regular intervals. Determine, if")
            .c("this frame is one we need to make an update in. If not, we're")
            .c("done.")
            .v(30)
            .w("remainder")
            .w("return_if_non_zero")
            .c("Time for more updates!")
            .w("should_game_run")
            .w("load")
            .w("return_if_zero")
            .w("handle_input")
            .v(0)
            .w("drop")
            .w("update_positions");
    });

    // Game state - tile field size
    source.define("is_out_of_bounds", |s| {
        s.c("Compare x coordinate against lower bound.")
            .v(0)
            .v(2)
            .w("copy")
            .w("greater")
            .v(0)
            .w("copy")
            .w("return_if_non_zero")
            .v(0)
            .w("drop")
            .c("Compare y coordinate against lower bound.")
            .v(0)
            .v(1)
            .w("copy")
            .w("greater")
            .v(0)
            .w("copy")
            .w("return_if_non_zero")
            .v(0)
            .w("drop")
            .c("Compare x coordinate against upper bound")
            .v(1)
            .w("take")
            .w("tile_field_size")
            .w("x")
            .w("load")
            .v(1)
            .w("sub")
            .w("greater")
            .v(0)
            .w("copy")
            .w("return_if_non_zero")
            .v(0)
            .w("drop")
            .c("Compare y coordinate against upper bound")
            .w("tile_field_size")
            .w("y")
            .w("load")
            .v(1)
            .w("sub")
            .w("greater");
    });

    // Game state - frame count
    source.define("init_frame_count", |s| {
        s.v(1).w("frame_count").w("store");
    });
    source.define("update_frame_count", |s| {
        s
            .c("We only have 7 bits to count (our 8-bit values are signed), so")
            .c("we need to reset the count every so often. To keep things")
            .c("predictable, let's reset only at full seconds. Assuming 60")
            .c("frames per second, `120` is the highest number we can count up")
            .c("to.")
            .c("")
            .c("Since we start counting at `1`, we need to reset *after* we")
            .c("reach that number, or we won't reset on a full second. Let's")
            .c("prepare the number to compare to for later use.")
            .v(121)
            .c("Grab the current frame count.")
            .w("frame_count")
            .w("load")
            .c("Increment the frame count.")
            .v(1)
            .w("add")
            .c("Place a copy of the new frame count back where it came from.")
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

    // Game state - should game run
    source.define("init_should_game_run", |s| {
        s.v(1).w("should_game_run").w("store");
    });

    // Game state - velocity
    source.define("init_velocity", |s| {
        s.v(1).v(0).w("velocity").w("store_vector");
    });

    // Game state - next position
    source.define("init_next_position", |s| {
        s.v(0)
            .w("position")
            .w("vec_load")
            .w("next_position")
            .w("store_vector");
    });
    source.define("update_next_position", |s| {
        s.v(0)
            .w("position")
            .w("x")
            .w("load")
            .w("velocity")
            .w("x")
            .w("load")
            .w("add")
            .v(0)
            .w("position")
            .w("y")
            .w("load")
            .w("velocity")
            .w("y")
            .w("load")
            .w("add")
            .w("next_position")
            .w("store_vector");
    });

    // Game state - positions
    source.define("position", |s| {
        s.b(["index"])
            .w("positions_first")
            .w("load")
            .b(["first"])
            .w("positions_buffer")
            .w("first")
            .w("add")
            .w("index")
            .w("add");
    });
    source.define("init_positions", |s| {
        s.v(0)
            .w("positions_first")
            .w("store")
            .v(15)
            .v(15)
            .w("positions_buffer")
            .w("store_vector");
    });
    source.define("update_positions", |s| {
        s.w("update_next_position")
            .w("next_position")
            .w("vec_load")
            .w("is_out_of_bounds")
            .w("return_if_non_zero")
            .w("next_position")
            .w("vec_load")
            .w("positions_buffer")
            .w("store_vector");
    });

    // Input
    source.define("handle_input", |s| {
        s.c("This function handles a single input event, so the absence of")
            .c("any recursive calls is by design. The next input event should")
            .c("only be applied, after the effects of the current one have")
            .c("been processed.")
            .c("")
            .c("This call returns a number with the following")
            .c("meaning:")
            .c("- 0: No input available.")
            .c("- 1: up")
            .c("- 2: left")
            .c("- 3: down")
            .c("- 4: right")
            .w("read_input")
            .c("Return, if no input is available.")
            .v(0)
            .w("copy")
            .w("return_if_zero")
            .c("Assume result was `1`, and apply an `up` event.")
            .v(0)
            .v(-1)
            .w("velocity")
            .w("store_vector")
            .c("Now check if it actually was an Up event, and if so, return.")
            .v(1)
            .w("sub")
            .v(0)
            .w("copy")
            .w("return_if_zero")
            .c("Seems it wasn't `up`. Try again for `left`.")
            .v(-1)
            .v(0)
            .w("velocity")
            .w("store_vector")
            .v(1)
            .w("sub")
            .v(0)
            .w("copy")
            .w("return_if_zero")
            .c("It wasn't `left` either. Re-try for `down`.")
            .v(0)
            .v(1)
            .w("velocity")
            .w("store_vector")
            .v(1)
            .w("sub")
            .v(0)
            .w("copy")
            .w("return_if_zero")
            .c("Guessed wrong again. One more try for `right`.")
            .v(1)
            .v(0)
            .w("velocity")
            .w("store_vector")
            .v(1)
            .w("sub")
            .v(0)
            .w("copy")
            .w("return_if_zero")
            .c("It seems it wasn't that either, which means we received an")
            .c("invalid return value. This would be a good place to trigger a")
            .c("panic, but the language doesn't support that yet.");
    });

    // Memory map
    source.define("tile_field_size", |s| {
        s.c("Address of the tile field height in memory.").v(0);
    });
    source.define("frame_count", |s| {
        s.c("Address of the frame count in memory.").v(2);
    });
    source.define("should_game_run", |s| {
        s.v(3);
    });
    source.define("velocity", |s| {
        s.c("Address of the velocity vector in memory").v(4);
    });
    source.define("next_position", |s| {
        s.c("Address of the next position vector in memory").v(6);
    });
    source.define("positions_first", |s| {
        s.v(8);
    });
    source.define("positions_buffer", |s| {
        s.c("Address of the position vector in memory").v(11);
    });

    // Vectors
    source.define("x", |s| {
        s.c("Offset of x coordinate within vector is zero. Nothing to do")
            .c("here.");
    });
    source.define("y", |s| {
        s.c("Offset of y coordinate within vector.").v(1).w("add");
    });
    source.define("vec_load", |s| {
        s.c("Make a copy of the vector address, since we're going to need")
            .c("it for each coordinate.")
            .v(0)
            .w("copy")
            .c("Load x coordinate.")
            .w("x")
            .w("load")
            .c("Get that copy of the vector address that we made.")
            .v(1)
            .w("take")
            .c("Load y coordinate.")
            .w("y")
            .w("load");
    });
    source.define("store_vector", |s| {
        s.c("Make a copy of the vector address.")
            .v(0)
            .w("copy")
            .c("Place one copy of the vector address next to the x coordinate")
            .c("value, so both coordinate values have the address next to")
            .c("them.")
            .v(2)
            .w("place")
            .c("Everything is prepared. We can just store the coordinate now")
            .w("y")
            .w("store")
            .w("x")
            .w("store");
    });
    source.define("copy_vector", |s| {
        s.v(1).w("copy").v(1).w("copy");
    });
    source.define("drop_vector", |s| {
        s.v(0).w("drop").v(0).w("drop");
    });

    source.compile("main")
}
