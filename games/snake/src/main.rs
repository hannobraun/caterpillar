use capi_compiler::repr::syntax::Script;

pub fn main() {
    let mut script = Script::default();
    snake(&mut script);

    let script = ron::to_string(&script).unwrap();
    println!("{script}");
}

fn snake(script: &mut Script) {
    // Main loop
    script.function("main", [], |s| {
        s.r("tile_field_size")
            .r("vec_store")
            .r("init_frame_count")
            .r("init")
            .r("main_inner");
    });
    script.function("main_inner", [], |s| {
        s.r("draw").r("count_frame").r("update").r("main_inner");
    });

    // Draw
    script.function("draw", [], |s| {
        s.r("clear_pixels")
            .r("draw_snake")
            .r("draw_food")
            .c("This blocks until the display system is ready to process the")
            .c("next frame.")
            .r("submit_frame");
    });
    script.function("draw_snake", [], |s| {
        s.v(0).r("draw_snake_inner");
    });
    script.function("draw_snake_inner", ["index"], |s| {
        s.r("positions")
            .r("index")
            .r("vec_buf_get")
            .v(0)
            .v(255)
            .v(0)
            .v(255)
            .r("set_pixel")
            .r("positions")
            .r("vec_buf_len")
            .r("index")
            .v(1)
            .r("add")
            .r("sub")
            .r("return_if_zero")
            .r("index")
            .v(1)
            .r("add")
            .r("draw_snake_inner");
    });
    script.function("draw_food", [], |s| {
        s.r("food_position")
            .r("vec_load")
            .v(255)
            .v(0)
            .v(0)
            .v(255)
            .r("set_pixel");
    });

    // Draw - clear pixels
    script.function("clear_pixels", [], |s| {
        s.r("init_tile_index").r("clear_pixels_inner").r("vec_drop");
    });
    script.function("clear_pixels_inner", ["tile_x", "tile_y"], |s| {
        s
            .c("This is a recursive function, so we might have been at it for")
            .c("a while, if we make it here. Check if the tile index has gone")
            .c("beyond the last tile, which would let us know that we're done.")
            .r("tile_x")
            .r("tile_y")
            .r("copy")
            .r("check_tile_index")
            .r("return_if_zero")
            .c("Apparently we're not done yet.")
            .r("vec_copy")
            .v(0)
            .v(0)
            .v(0)
            .v(255)
            .r("set_pixel")
            .r("increment_tile_index")
            .r("clear_pixels_inner");
    });

    // Draw - write tiles - tile index
    script.function("init_tile_index", [], |s| {
        s.v(0).v(0);
    });
    script.function("check_tile_index", ["tile_y"], |s| {
        s.r("tile_field_size")
            .r("vec_load")
            .r("vec_y")
            .r("tile_y")
            .c("Leave zero, if the y-coordinate has advanced beyond the last")
            .c("line of the tile field. Otherwise, leave non-zero value.")
            .r("sub");
    });
    script.function("increment_tile_index", ["tile_x", "tile_y"], |s| {
        s.c("Increment the x-coordinate.")
            .r("tile_x")
            .v(1)
            .r("add")
            .bind(["tile_x_new"])
            .c("Check if the x coordinate has advanced beyond the width.")
            .r("tile_field_size")
            .r("vec_load")
            .r("vec_x")
            .r("tile_x_new")
            .r("sub")
            .bind(["zero_if_x_overflowed"])
            .c("Unless the x-coordinate has advanced beyond the width, we're")
            .c("done here.")
            .r("tile_x_new")
            .r("tile_y")
            .r("zero_if_x_overflowed")
            .r("return_if_non_zero")
            .c("Looks like we're not done!")
            .bind(["tile_x_new", "tile_y"])
            .c("Increment y-coordinate.")
            .r("tile_y")
            .v(1)
            .r("add")
            .bind(["tile_y_new"])
            .c("Return updated coordinates")
            .v(0)
            .r("tile_y_new");
    });

    // Tile field size
    script.function("is_out_of_bounds", ["x", "y"], |s| {
        s.c("Compare x coordinate against lower bound.")
            .v(0)
            .r("x")
            .r("greater")
            .r("copy")
            .r("return_if_non_zero")
            .r("drop")
            .c("Compare y coordinate against lower bound.")
            .v(0)
            .r("y")
            .r("greater")
            .r("copy")
            .r("return_if_non_zero")
            .r("drop")
            .c("Compare x coordinate against upper bound")
            .r("x")
            .r("tile_field_size")
            .r("vec_load")
            .r("vec_x")
            .v(1)
            .r("sub")
            .r("greater")
            .r("copy")
            .r("return_if_non_zero")
            .r("drop")
            .c("Compare y coordinate against upper bound")
            .r("y")
            .r("tile_field_size")
            .r("vec_load")
            .r("vec_y")
            .v(1)
            .r("sub")
            .r("greater");
    });

    // Frame count
    script.function("init_frame_count", [], |s| {
        s.v(1).r("frame_count").r("store");
    });
    script.function("count_frame", [], |s| {
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
            .r("frame_count")
            .r("load")
            .c("Increment the frame count.")
            .v(1)
            .r("add")
            .c("Place a copy of the new frame count back where it came from.")
            .r("copy")
            .r("frame_count")
            .r("store")
            .c("We have a copy of the new frame count left on the top of the")
            .c("stack. Let's see if we counted up to the maximum value. If")
            .c("not, we're done.")
            .r("sub")
            .r("return_if_non_zero")
            .c("We have counted up to the maximum value. Reset the frame")
            .c("count.")
            .r("init_frame_count");
    });

    // Game state
    script.function("init", [], |s| {
        s.r("init_should_game_run")
            .r("snake_init")
            .r("init_velocity")
            .r("init_next_position")
            .r("food_init");
    });
    script.function("update", [], |s| {
        s.c("The update logic does not run every frame.")
            .r("frame_count")
            .r("load")
            .v(5)
            .r("remainder")
            .r("return_if_non_zero")
            .c("Looks like it's time to run updates!")
            .r("should_game_run")
            .r("load")
            .r("return_if_zero")
            .r("handle_input")
            .r("drop")
            .r("update_positions")
            .r("food_eat");
    });

    // Game state - should game run
    script.function("init_should_game_run", [], |s| {
        s.v(1).r("should_game_run").r("store");
    });

    // Game state - velocity
    script.function("init_velocity", [], |s| {
        s.v(1).v(0).r("velocity").r("vec_store");
    });

    // Game state - next position
    script.function("init_next_position", [], |s| {
        s.r("positions")
            .v(0)
            .r("vec_buf_get")
            .r("next_position")
            .r("vec_store");
    });
    script.function("update_next_position", [], |s| {
        s.r("snake_head")
            .r("vec_x")
            .r("velocity")
            .r("vec_load")
            .r("vec_x")
            .r("add")
            .r("snake_head")
            .r("vec_y")
            .r("velocity")
            .r("vec_load")
            .r("vec_y")
            .r("add")
            .r("next_position")
            .r("vec_store")
            .r("next_position")
            .r("vec_load")
            .r("is_out_of_bounds")
            .r("return_if_zero")
            .r("next_position")
            .r("vec_load")
            .bind(["next_x", "next_y"])
            .r("tile_field_size")
            .r("vec_load")
            .bind(["limit_x", "limit_y"])
            .r("next_x")
            .r("limit_x")
            .r("handle_coordinate_smaller_than_zero")
            .bind(["next_x"])
            .r("next_y")
            .r("limit_y")
            .r("handle_coordinate_smaller_than_zero")
            .bind(["next_y"])
            .r("next_x")
            .r("limit_x")
            .r("handle_coordinate_larger_than_limit")
            .bind(["next_x"])
            .r("next_y")
            .r("limit_y")
            .r("handle_coordinate_larger_than_limit")
            .bind(["next_y"])
            .r("next_x")
            .r("next_y")
            .r("next_position")
            .r("vec_store");
    });
    script.function(
        "handle_coordinate_smaller_than_zero",
        ["coord", "limit"],
        |s| {
            s.v(0)
                .r("coord")
                .r("greater")
                .bind(["coord_smaller_than_zero"])
                .r("coord")
                .r("coord_smaller_than_zero")
                .r("return_if_zero")
                .r("limit")
                .r("add");
        },
    );
    script.function(
        "handle_coordinate_larger_than_limit",
        ["coord", "limit"],
        |s| {
            s.r("limit")
                .r("coord")
                .r("greater")
                .bind(["limit_greater_than_coord"])
                .r("coord")
                .r("limit_greater_than_coord")
                .r("return_if_non_zero")
                .r("limit")
                .r("sub");
        },
    );

    // Game state - food
    script.function("food_init", [], |s| {
        s.r("negatable_random")
            .r("abs")
            .r("tile_field_size")
            .r("vec_load")
            .r("vec_x")
            .r("remainder")
            .r("negatable_random")
            .r("abs")
            .r("tile_field_size")
            .r("vec_load")
            .r("vec_y")
            .r("remainder")
            .r("food_position")
            .r("vec_store");
    });
    script.function("food_eat", [], |s| {
        s.r("_food_collides_with_snake")
            .r("return_if_zero")
            .c("The snake's head and the food are at the same position.")
            .r("food_init")
            .r("grow_snake");
    });
    script.function("_food_collides_with_snake", [], |s| {
        s.r("snake_head")
            .r("food_position")
            .r("vec_load")
            .r("vec_eq")
            .bind(["head_collides"])
            .r("food_position")
            .r("vec_load")
            .r("check_body_collision")
            .bind(["body_collides"])
            .r("head_collides")
            .r("body_collides")
            .r("add")
            .v(0)
            .r("greater");
    });

    // Game state - snake
    script.function("snake_init", [], |s| {
        s.v(3)
            .r("snake_length")
            .r("store")
            .r("positions")
            .r("vec_buf_init")
            .r("positions")
            .v(15)
            .v(15)
            .r("vec_buf_push");
    });
    script.function("snake_head", [], |s| {
        s.r("positions").r("vec_buf_last");
    });
    script.function("update_positions", [], |s| {
        s.r("update_next_position")
            .r("snake_head")
            .r("check_body_collision")
            .r("return_if_non_zero")
            .r("positions")
            .r("next_position")
            .r("vec_load")
            .r("vec_buf_push")
            .r("pop_positions");
    });
    script.function("pop_positions", [], |s| {
        s.r("positions")
            .r("vec_buf_len")
            .r("snake_length")
            .r("load")
            .r("greater")
            .r("return_if_zero")
            .r("positions")
            .r("vec_buf_pop")
            .r("pop_positions");
    });
    script.function("grow_snake", [], |s| {
        s.r("snake_length")
            .r("load")
            .v(1)
            .r("add")
            .bind(["snake_length_plus_growth"])
            .r("snake_length_plus_growth")
            .r("positions")
            .r("vec_buf_capacity")
            .r("greater")
            .r("return_if_non_zero")
            .r("snake_length_plus_growth")
            .r("snake_length")
            .r("store");
    });
    script.function("check_body_collision", ["x", "y"], |s| {
        s.r("x").r("y").v(0).r("check_body_collision_inner");
    });
    script.function("check_body_collision_inner", ["x", "y", "index"], |s| {
        s.r("positions")
            .r("vec_buf_len")
            .v(1)
            .r("sub")
            .r("index")
            .r("greater")
            .r("copy")
            .r("return_if_zero")
            .r("drop")
            .r("positions")
            .r("index")
            .r("vec_buf_get")
            .r("vec_x")
            .r("x")
            .r("eq")
            .bind(["x_matches"])
            .r("positions")
            .r("index")
            .r("vec_buf_get")
            .r("vec_y")
            .r("y")
            .r("eq")
            .bind(["y_matches"])
            .r("x_matches")
            .r("y_matches")
            .r("add")
            .v(2)
            .r("eq")
            .r("copy")
            .r("return_if_non_zero")
            .r("drop")
            .r("x")
            .r("y")
            .r("index")
            .v(1)
            .r("add")
            .r("check_body_collision_inner");
    });

    // Input
    script.function("handle_input", [], |s| {
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
            .r("read_input")
            .c("Return, if no input is available.")
            .r("copy")
            .r("return_if_zero")
            .c("Assume result was `1`, and apply an `up` event.")
            .v(0)
            .v(-1)
            .r("velocity")
            .r("vec_store")
            .c("Now check if it actually was an `up` event, and if so, return.")
            .v(1)
            .r("sub")
            .r("copy")
            .r("return_if_zero")
            .c("Seems it wasn't `up`. Try again for `left`.")
            .v(-1)
            .v(0)
            .r("velocity")
            .r("vec_store")
            .v(1)
            .r("sub")
            .r("copy")
            .r("return_if_zero")
            .c("It wasn't `left` either. Re-try for `down`.")
            .v(0)
            .v(1)
            .r("velocity")
            .r("vec_store")
            .v(1)
            .r("sub")
            .r("copy")
            .r("return_if_zero")
            .c("Guessed wrong again. One more try for `right`.")
            .v(1)
            .v(0)
            .r("velocity")
            .r("vec_store")
            .v(1)
            .r("sub")
            .r("copy")
            .c("If it wasn't `right` either, this would be a good place to")
            .c("trigger a panic. But the language doesn't support that yet.");
    });

    // Memory map
    script.function("tile_field_size", [], |s| {
        s.v(0);
    });
    script.function("frame_count", [], |s| {
        s.v(2);
    });
    script.function("should_game_run", [], |s| {
        s.v(3);
    });
    script.function("velocity", [], |s| {
        s.v(4);
    });
    script.function("next_position", [], |s| {
        s.v(6);
    });
    script.function("food_position", [], |s| {
        s.v(8);
    });
    script.function("snake_length", [], |s| {
        s.v(10);
    });
    script.function("positions", [], |s| {
        s.v(11);
    });

    // Utilities - Vector
    script.function("vec_x", ["x", "_"], |s| {
        s.r("x");
    });
    script.function("vec_y", ["_", "y"], |s| {
        s.r("y");
    });
    script.function("vec_load", ["address"], |s| {
        s.r("address")
            .r("load")
            .r("address")
            .v(1)
            .r("add")
            .r("load");
    });
    script.function("vec_store", ["x", "y", "address"], |s| {
        s.r("x")
            .r("address")
            .r("store")
            .r("y")
            .r("address")
            .v(1)
            .r("add")
            .r("store");
    });
    script.function("vec_copy", ["vx", "vy"], |s| {
        s.r("vx").r("vy").r("vx").r("vy");
    });
    script.function("vec_drop", ["_", "_"], |_| {});
    script.function("vec_eq", ["ax", "ay", "bx", "by"], |s| {
        s.r("ax")
            .r("bx")
            .r("eq")
            .r("copy")
            .r("return_if_zero")
            .r("drop")
            .r("ay")
            .r("by")
            .r("eq")
            .r("copy")
            .r("return_if_zero")
            .r("drop")
            .c("Vectors are equal!")
            .v(1);
    });

    // Utilities - Vector Buffer
    script.function("vec_buf_init", ["vec_buf"], |s| {
        s.v(0)
            .r("vec_buf")
            .r("_vec_buf_first")
            .r("store")
            .v(0)
            .r("vec_buf")
            .r("_vec_buf_next")
            .r("store")
            .v(64)
            .r("vec_buf")
            .r("_vec_buf_capacity")
            .r("store");
    });
    script.function("vec_buf_get", ["vec_buf", "index"], |s| {
        s.r("index")
            .v(2)
            .r("mul")
            .bind(["offset"])
            .r("vec_buf")
            .r("_vec_buf_first")
            .r("load")
            .bind(["base"])
            .r("vec_buf")
            .r("base")
            .r("offset")
            .r("_vec_buf_address")
            .r("vec_load");
    });
    script.function("vec_buf_last", ["vec_buf"], |s| {
        s.r("vec_buf")
            .r("vec_buf_len")
            .v(1)
            .r("sub")
            .bind(["index"])
            .r("vec_buf")
            .r("index")
            .r("vec_buf_get");
    });
    script.function("vec_buf_push", ["vec_buf", "x", "y"], |s| {
        s.r("vec_buf")
            .r("_vec_buf_next")
            .bind(["next_addr"])
            .r("vec_buf")
            .r("next_addr")
            .r("load")
            .v(0)
            .r("_vec_buf_address")
            .bind(["address"])
            .r("x")
            .r("y")
            .r("address")
            .r("vec_store")
            .r("next_addr")
            .r("_vec_buf_inc_index");
    });
    script.function("vec_buf_pop", ["vec_buf"], |s| {
        s.r("vec_buf").r("_vec_buf_first").r("_vec_buf_inc_index");
    });
    script.function("vec_buf_len", ["vec_buf"], |s| {
        s.r("vec_buf")
            .r("_vec_buf_first")
            .r("load")
            .bind(["first"])
            .r("vec_buf")
            .r("_vec_buf_next")
            .r("load")
            .bind(["next"])
            .r("next")
            .r("first")
            .r("sub")
            .v(2)
            .r("div")
            .bind(["difference"])
            .r("difference")
            .r("difference")
            .r("return_if_zero")
            .v(0)
            .r("difference")
            .r("greater")
            .r("return_if_zero")
            .r("vec_buf")
            .r("_vec_buf_capacity")
            .r("load")
            .r("add");
    });
    script.function("vec_buf_capacity", ["vec_buf"], |s| {
        s.r("vec_buf")
            .r("_vec_buf_capacity")
            .r("load")
            .v(2)
            .r("div");
    });
    script.function("_vec_buf_address", ["vec_buf", "base", "offset"], |s| {
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
            .r("base")
            .r("offset")
            .r("add_wrap_unsigned")
            .r("vec_buf")
            .r("_vec_buf_capacity")
            .r("load")
            .r("remainder")
            .r("vec_buf")
            .r("_vec_buf_buffer")
            .r("add_wrap_unsigned");
    });
    script.function("_vec_buf_inc_index", ["index_addr"], |s| {
        s.r("index_addr")
            .r("load")
            .v(2)
            .r("add_wrap_unsigned")
            .r("index_addr")
            .r("store");
    });
    script.function("_vec_buf_first", ["vec_buf"], |s| {
        s.r("vec_buf").v(0).r("add");
    });
    script.function("_vec_buf_next", ["vec_buf"], |s| {
        s.r("vec_buf").v(1).r("add");
    });
    script.function("_vec_buf_capacity", ["vec_buf"], |s| {
        s.r("vec_buf").v(2).r("add");
    });
    script.function("_vec_buf_buffer", ["vec_buf"], |s| {
        s.r("vec_buf").v(3).r("add");
    });

    // Utilities - Miscellaneous
    script.function("negatable_random", [], |s| {
        s.c("Negating the minimum number would result in an integer overflow.")
            .r("read_random")
            .r("copy")
            .r("word_min")
            .r("eq")
            .r("return_if_zero")
            .r("drop")
            .c("Looks like we ran into the minimum. Try again!")
            .r("negatable_random");
    });
    script.function("abs", ["v"], |s| {
        s.r("v")
            .r("v")
            .v(-1)
            .r("greater")
            .r("return_if_non_zero")
            .r("neg");
    });

    // Utilities - Words
    script.function("word_min", [], |s| {
        s.v(i32::MIN);
    });
}
