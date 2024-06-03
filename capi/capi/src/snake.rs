use capi_runtime::{Program, Source};

pub fn program() -> Program {
    let mut source = Source::default();

    // Main loop
    source.function("main", |s| {
        s.w("tile_field_size")
            .w("vec_store")
            .w("init_frame_count")
            .w("init")
            .w("main_inner");
    });
    source.function("main_inner", |s| {
        s.w("draw").w("count_frame").w("update").w("main_inner");
    });

    // Draw
    source.function("draw", |s| {
        s.w("clear_all_tiles")
            .w("draw_snake")
            .w("draw_food")
            .c("This blocks until the display system is ready to process the")
            .c("next frame.")
            .w("submit_frame");
    });
    source.function("clear_all_tiles", |s| {
        s.v(0).w("write_all_tiles");
    });
    source.function("draw_snake", |s| {
        s.v(0).w("draw_snake_inner");
    });
    source.function("draw_snake_inner", |s| {
        s.b(["index"])
            .w("positions")
            .w("index")
            .w("vec_buf_get")
            .w("vec_load")
            .v(1)
            .w("write_tile")
            .w("positions")
            .w("vec_buf_len")
            .w("index")
            .v(1)
            .w("add")
            .w("sub")
            .w("return_if_zero")
            .w("index")
            .v(1)
            .w("add")
            .w("draw_snake_inner");
    });
    source.function("draw_food", |s| {
        s.w("food_position").w("vec_load").v(1).w("write_tile");
    });

    // Draw - write tiles
    source.function("write_all_tiles", |s| {
        s.b(["tile_value"])
            .w("init_tile_index")
            .w("tile_value")
            .w("write_all_tiles_inner")
            .w("vec_drop");
    });
    source.function("write_all_tiles_inner", |s| {
        s.b(["tile_value"])
            .c("This is a recursive function, so we might have been at it for")
            .c("a while, if we make it here. Check if the tile index has gone")
            .c("beyond the last tile, which would let us know that we're done.")
            .v(0)
            .w("copy")
            .w("check_tile_index")
            .w("return_if_zero")
            .c("Apparently we're not done yet.")
            .w("vec_copy")
            .w("tile_value")
            .w("write_tile")
            .w("increment_tile_index")
            .w("tile_value")
            .w("write_all_tiles_inner");
    });

    // Draw - write tiles - tile index
    source.function("init_tile_index", |s| {
        s.v(0).v(0);
    });
    source.function("check_tile_index", |s| {
        s.b(["tile_y"])
            .w("tile_field_size")
            .w("vec_y")
            .w("load")
            .w("tile_y")
            .c("Leave zero, if the y-coordinate has advanced beyond the last")
            .c("line of the tile field. Otherwise, leave non-zero value.")
            .w("sub");
    });
    source.function("increment_tile_index", |s| {
        s.b(["tile_x", "tile_y"])
            .c("Increment the x-coordinate.")
            .w("tile_x")
            .v(1)
            .w("add")
            .b(["tile_x_new"])
            .c("Check if the x coordinate has advanced beyond the width.")
            .w("tile_field_size")
            .w("vec_x")
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

    // Tile field size
    source.function("is_out_of_bounds", |s| {
        s.c("Compare x coordinate against lower bound.")
            .v(0)
            .v(2)
            .w("copy")
            .w("greater")
            .v(0)
            .w("copy")
            .w("return_if_non_zero")
            .w("drop")
            .c("Compare y coordinate against lower bound.")
            .v(0)
            .v(1)
            .w("copy")
            .w("greater")
            .v(0)
            .w("copy")
            .w("return_if_non_zero")
            .w("drop")
            .c("Compare x coordinate against upper bound")
            .v(1)
            .w("take")
            .w("tile_field_size")
            .w("vec_x")
            .w("load")
            .v(1)
            .w("sub")
            .w("greater")
            .v(0)
            .w("copy")
            .w("return_if_non_zero")
            .w("drop")
            .c("Compare y coordinate against upper bound")
            .w("tile_field_size")
            .w("vec_y")
            .w("load")
            .v(1)
            .w("sub")
            .w("greater");
    });

    // Frame count
    source.function("init_frame_count", |s| {
        s.v(1).w("frame_count").w("store");
    });
    source.function("count_frame", |s| {
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

    // Game state
    source.function("init", |s| {
        s.w("init_should_game_run")
            .w("init_snake")
            .w("init_velocity")
            .w("init_next_position")
            .w("init_food");
    });
    source.function("update", |s| {
        s.c("The update logic does not run every frame.")
            .w("frame_count")
            .w("load")
            .v(5)
            .w("remainder")
            .w("return_if_non_zero")
            .c("Looks like it's time to run updates!")
            .w("should_game_run")
            .w("load")
            .w("return_if_zero")
            .w("handle_input")
            .w("drop")
            .w("update_positions")
            .w("eat_food");
    });

    // Game state - should game run
    source.function("init_should_game_run", |s| {
        s.v(1).w("should_game_run").w("store");
    });

    // Game state - velocity
    source.function("init_velocity", |s| {
        s.v(1).v(0).w("velocity").w("vec_store");
    });

    // Game state - next position
    source.function("init_next_position", |s| {
        s.w("positions")
            .v(0)
            .w("vec_buf_get")
            .w("vec_load")
            .w("next_position")
            .w("vec_store");
    });
    source.function("update_next_position", |s| {
        s.w("positions")
            .w("vec_buf_last")
            .w("vec_x")
            .w("load")
            .w("velocity")
            .w("vec_x")
            .w("load")
            .w("add")
            .w("positions")
            .w("vec_buf_last")
            .w("vec_y")
            .w("load")
            .w("velocity")
            .w("vec_y")
            .w("load")
            .w("add")
            .w("next_position")
            .w("vec_store");
    });

    // Game state - food
    source.function("init_food", |s| {
        s.w("negatable_random")
            .w("abs")
            .w("tile_field_size")
            .w("vec_x")
            .w("load")
            .w("remainder")
            .w("negatable_random")
            .w("abs")
            .w("tile_field_size")
            .w("vec_y")
            .w("load")
            .w("remainder")
            .w("food_position")
            .w("vec_store");
    });
    source.function("eat_food", |s| {
        s.w("positions")
            .w("vec_buf_last")
            .w("vec_load")
            .w("food_position")
            .w("vec_load")
            .w("vec_eq")
            .w("return_if_zero")
            .c("The snake's head and the food are at the same position.")
            .w("init_food")
            .w("grow_snake");
    });

    // Game state - snake
    source.function("init_snake", |s| {
        s.v(3)
            .w("snake_length")
            .w("store")
            .w("positions")
            .w("vec_buf_init")
            .w("positions")
            .v(15)
            .v(15)
            .w("vec_buf_push");
    });
    source.function("update_positions", |s| {
        s.w("update_next_position")
            .w("next_position")
            .w("vec_load")
            .w("is_out_of_bounds")
            .w("return_if_non_zero")
            .w("positions")
            .w("next_position")
            .w("vec_load")
            .w("vec_buf_push")
            .w("pop_positions");
    });
    source.function("pop_positions", |s| {
        s.w("positions")
            .w("vec_buf_len")
            .w("snake_length")
            .w("load")
            .w("greater")
            .w("return_if_zero")
            .w("positions")
            .w("vec_buf_pop")
            .w("pop_positions");
    });
    source.function("grow_snake", |s| {
        s.w("snake_length")
            .w("load")
            .v(1)
            .w("add")
            .b(["snake_length_plus_growth"])
            .w("snake_length_plus_growth")
            .w("positions")
            .w("vec_buf_capacity")
            .w("greater")
            .w("return_if_non_zero")
            .w("snake_length_plus_growth")
            .w("snake_length")
            .w("store");
    });

    // Input
    source.function("handle_input", |s| {
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
            .w("vec_store")
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
            .w("vec_store")
            .v(1)
            .w("sub")
            .v(0)
            .w("copy")
            .w("return_if_zero")
            .c("It wasn't `left` either. Re-try for `down`.")
            .v(0)
            .v(1)
            .w("velocity")
            .w("vec_store")
            .v(1)
            .w("sub")
            .v(0)
            .w("copy")
            .w("return_if_zero")
            .c("Guessed wrong again. One more try for `right`.")
            .v(1)
            .v(0)
            .w("velocity")
            .w("vec_store")
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
    source.function("tile_field_size", |s| {
        s.v(0);
    });
    source.function("frame_count", |s| {
        s.v(2);
    });
    source.function("should_game_run", |s| {
        s.v(3);
    });
    source.function("velocity", |s| {
        s.v(4);
    });
    source.function("next_position", |s| {
        s.v(6);
    });
    source.function("food_position", |s| {
        s.v(8);
    });
    source.function("snake_length", |s| {
        s.v(10);
    });
    source.function("positions", |s| {
        s.v(11);
    });

    // Utilities - Vector
    source.function("vec_x", |s| {
        s.c("Offset of x coordinate within vector is zero. Nothing to do")
            .c("here.");
    });
    source.function("vec_y", |s| {
        s.c("Offset of y coordinate within vector.").v(1).w("add");
    });
    source.function("vec_load", |s| {
        s.c("Make a copy of the vector address, since we're going to need")
            .c("it for each coordinate.")
            .v(0)
            .w("copy")
            .c("Load x coordinate.")
            .w("vec_x")
            .w("load")
            .c("Get that copy of the vector address that we made.")
            .v(1)
            .w("take")
            .c("Load y coordinate.")
            .w("vec_y")
            .w("load");
    });
    source.function("vec_store", |s| {
        s.b(["x", "y", "addr"])
            .w("x")
            .w("addr")
            .w("store")
            .w("y")
            .w("addr")
            .v(1)
            .w("add")
            .w("store");
    });
    source.function("vec_copy", |s| {
        s.b(["vx", "vy"]).w("vx").w("vy").w("vx").w("vy");
    });
    source.function("vec_drop", |s| {
        s.w("drop").w("drop");
    });
    source.function("vec_eq", |s| {
        s.b(["ax", "ay", "bx", "by"])
            .w("ax")
            .w("bx")
            .w("eq")
            .v(0)
            .w("copy")
            .w("return_if_zero")
            .w("ay")
            .w("by")
            .w("eq")
            .v(0)
            .w("copy")
            .w("return_if_zero")
            .w("drop")
            .c("Vectors are equal!")
            .v(1);
    });

    // Utilities - Vector Buffer
    source.function("vec_buf_init", |s| {
        s.b(["vec_buf"])
            .v(0)
            .w("vec_buf")
            .w("_vec_buf_first")
            .w("store")
            .v(0)
            .w("vec_buf")
            .w("_vec_buf_next")
            .w("store")
            .v(64)
            .w("vec_buf")
            .w("_vec_buf_capacity")
            .w("store");
    });
    source.function("vec_buf_get", |s| {
        s.b(["vec_buf", "index"])
            .w("index")
            .v(2)
            .w("mul")
            .b(["offset"])
            .w("vec_buf")
            .w("_vec_buf_first")
            .w("load")
            .b(["base"])
            .w("vec_buf")
            .w("base")
            .w("offset")
            .w("_vec_buf_address");
    });
    source.function("vec_buf_last", |s| {
        s.b(["vec_buf"])
            .w("vec_buf")
            .w("vec_buf_len")
            .v(1)
            .w("sub")
            .b(["index"])
            .w("vec_buf")
            .w("index")
            .w("vec_buf_get");
    });
    source.function("vec_buf_push", |s| {
        s.b(["vec_buf", "x", "y"])
            .w("vec_buf")
            .w("_vec_buf_next")
            .b(["next_addr"])
            .w("vec_buf")
            .w("next_addr")
            .w("load")
            .v(0)
            .w("_vec_buf_address")
            .b(["address"])
            .w("x")
            .w("y")
            .w("address")
            .w("vec_store")
            .w("next_addr")
            .w("_vec_buf_inc_index");
    });
    source.function("vec_buf_pop", |s| {
        s.w("_vec_buf_first").w("_vec_buf_inc_index");
    });
    source.function("vec_buf_len", |s| {
        s.b(["vec_buf"])
            .w("vec_buf")
            .w("_vec_buf_first")
            .w("load")
            .b(["first"])
            .w("vec_buf")
            .w("_vec_buf_next")
            .w("load")
            .b(["next"])
            .w("next")
            .w("first")
            .w("sub")
            .v(2)
            .w("div")
            .b(["difference"])
            .w("difference")
            .w("difference")
            .w("return_if_zero")
            .v(0)
            .w("difference")
            .w("greater")
            .w("return_if_zero")
            .w("vec_buf")
            .w("_vec_buf_capacity")
            .w("load")
            .w("add");
    });
    source.function("vec_buf_capacity", |s| {
        s.w("_vec_buf_capacity").w("load").v(2).w("div");
    });
    source.function("_vec_buf_address", |s| {
        s.c("Compute the memory address of a location within the vector")
            .c("buffer.")
            .c("")
            .c("Takes two arguments:")
            .c("")
            .c("- `base`, which is an index into the buffer, as opposed to a")
            .c("  real address. It can be larger than any actual address")
            .c("  within the buffer.")
            .c("- `offset`, which is the offset of the desired address from")
            .c("  `base`.")
            .c("that it can ")
            .b(["vec_buf", "base", "offset"])
            .w("base")
            .w("offset")
            .w("add_wrap_unsigned")
            .w("vec_buf")
            .w("_vec_buf_capacity")
            .w("load")
            .w("remainder")
            .w("vec_buf")
            .w("_vec_buf_buffer")
            .w("add_wrap_unsigned");
    });
    source.function("_vec_buf_inc_index", |s| {
        s.b(["index_addr"])
            .w("index_addr")
            .w("load")
            .v(2)
            .w("add_wrap_unsigned")
            .w("index_addr")
            .w("store");
    });
    source.function("_vec_buf_first", |s| {
        s.v(0).w("add");
    });
    source.function("_vec_buf_next", |s| {
        s.v(1).w("add");
    });
    source.function("_vec_buf_capacity", |s| {
        s.v(2).w("add");
    });
    source.function("_vec_buf_buffer", |s| {
        s.v(3).w("add");
    });

    // Utilities - Miscellaneous
    source.function("negatable_random", |s| {
        s.c("Negating -128 would result in an integer overflow.")
            .w("read_random")
            .v(0)
            .w("copy")
            .v(-128)
            .w("eq")
            .w("return_if_zero")
            .w("drop")
            .c("Looks like we ran into -128. Try again!")
            .w("negatable_random");
    });
    source.function("abs", |s| {
        s.b(["v"])
            .w("v")
            .w("v")
            .v(-1)
            .w("greater")
            .w("return_if_non_zero")
            .w("neg");
    });

    source.compile("main")
}
