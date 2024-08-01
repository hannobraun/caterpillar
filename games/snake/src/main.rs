use capi_compiler::repr::syntax::Script;

pub fn main() {
    let mut script = Script::default();
    snake(&mut script);

    let script = ron::to_string(&script).unwrap();
    println!("{script}");
}

fn snake(script: &mut Script) {
    // Main loop
    script.function("main", ["size_x", "size_y"], |s| {
        s.ident("size_x")
            .ident("size_y")
            .ident("tile_field_size")
            .ident("vec_store")
            .ident("init_frame_count")
            .ident("init")
            .ident("main_inner");
    });
    script.function("main_inner", [], |s| {
        s.ident("draw")
            .ident("count_frame")
            .ident("update")
            .ident("main_inner");
    });

    // Draw
    script.function("draw", [], |s| {
        s.ident("clear_pixels")
            .ident("draw_snake")
            .ident("draw_food")
            .c("This blocks until the display system is ready to process the")
            .c("next frame.")
            .ident("submit_frame");
    });
    script.function("draw_snake", [], |s| {
        s.v(0).ident("draw_snake_inner");
    });
    script.function("draw_snake_inner", ["index"], |s| {
        s.ident("positions")
            .ident("index")
            .ident("vec_buf_get")
            .v(0)
            .v(255)
            .v(0)
            .v(255)
            .ident("set_pixel")
            .ident("positions")
            .ident("vec_buf_len")
            .ident("index")
            .v(1)
            .ident("add_i32")
            .ident("sub")
            .block(|s| {
                s.ident("index")
                    .v(1)
                    .ident("add_i32")
                    .ident("draw_snake_inner");
            })
            .block(|_| {})
            .ident("if");
    });
    script.function("draw_food", [], |s| {
        s.ident("food_position")
            .ident("vec_load")
            .v(255)
            .v(0)
            .v(0)
            .v(255)
            .ident("set_pixel");
    });

    // Draw - clear pixels
    script.function("clear_pixels", [], |s| {
        s.ident("init_tile_index").ident("clear_pixels_inner");
    });
    script.function("clear_pixels_inner", ["tile_x", "tile_y"], |s| {
        s
            .c("This is a recursive function, so we might have been at it for")
            .c("a while, if we make it here. Check if the tile index has gone")
            .c("beyond the last tile, which would let us know that we're done.")
            .ident("tile_y")
            .ident("check_tile_index")
            .block(|s| {
                s
                    .c("Apparently we're not done yet.")
                    .ident("tile_x")
                    .ident("tile_y")
                    .v(0)
                    .v(0)
                    .v(0)
                    .v(255)
                    .ident("set_pixel")
                    .ident("tile_x")
                    .ident("tile_y")
                    .ident("increment_tile_index")
                    .ident("clear_pixels_inner");
            })
            .block(|_| {})
            .ident("if");
    });

    // Draw - write tiles - tile index
    script.function("init_tile_index", [], |s| {
        s.v(0).v(0);
    });
    script.function("check_tile_index", ["tile_y"], |s| {
        s.ident("tile_field_size")
            .ident("vec_load")
            .ident("vec_y")
            .ident("tile_y")
            .c("Leave zero, if the y-coordinate has advanced beyond the last")
            .c("line of the tile field. Otherwise, leave non-zero value.")
            .ident("sub");
    });
    script.function("increment_tile_index", ["tile_x", "tile_y"], |s| {
        s.c("Increment the x-coordinate.")
            .ident("tile_x")
            .v(1)
            .ident("add_i32")
            .bind(["tile_x_new"])
            .c("Check if the x coordinate has advanced beyond the width.")
            .ident("tile_field_size")
            .ident("vec_load")
            .ident("vec_x")
            .ident("tile_x_new")
            .ident("sub")
            .bind(["zero_if_x_overflowed"])
            .c("Unless the x-coordinate has advanced beyond the width, we're")
            .c("done here.")
            .ident("tile_x_new")
            .ident("tile_y")
            .ident("zero_if_x_overflowed")
            .ident("return_if_non_zero")
            .c("Looks like we're not done!")
            .bind(["tile_x_new", "tile_y"])
            .c("Increment y-coordinate.")
            .ident("tile_y")
            .v(1)
            .ident("add_i32")
            .bind(["tile_y_new"])
            .c("Return updated coordinates")
            .v(0)
            .ident("tile_y_new");
    });

    // Tile field size
    script.function("is_out_of_bounds", ["x", "y"], |s| {
        s.c("Compare x coordinate against lower bound.")
            .v(0)
            .ident("x")
            .ident("greater")
            .ident("copy")
            .ident("return_if_non_zero")
            .ident("drop")
            .c("Compare y coordinate against lower bound.")
            .v(0)
            .ident("y")
            .ident("greater")
            .ident("copy")
            .ident("return_if_non_zero")
            .ident("drop")
            .c("Compare x coordinate against upper bound")
            .ident("x")
            .ident("tile_field_size")
            .ident("vec_load")
            .ident("vec_x")
            .v(1)
            .ident("sub")
            .ident("greater")
            .ident("copy")
            .ident("return_if_non_zero")
            .ident("drop")
            .c("Compare y coordinate against upper bound")
            .ident("y")
            .ident("tile_field_size")
            .ident("vec_load")
            .ident("vec_y")
            .v(1)
            .ident("sub")
            .ident("greater");
    });

    // Frame count
    script.function("init_frame_count", [], |s| {
        s.v(1).ident("frame_count").ident("store");
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
            .ident("frame_count")
            .ident("load")
            .c("Increment the frame count.")
            .v(1)
            .ident("add_i32")
            .c("Place a copy of the new frame count back where it came from.")
            .ident("copy")
            .ident("frame_count")
            .ident("store")
            .c("We have a copy of the new frame count left on the top of the")
            .c("stack. Let's see if we counted up to the maximum value. If")
            .c("not, we're done.")
            .ident("sub")
            .ident("return_if_non_zero")
            .c("We have counted up to the maximum value. Reset the frame")
            .c("count.")
            .ident("init_frame_count");
    });

    // Game state
    script.function("init", [], |s| {
        s.ident("init_should_game_run")
            .ident("snake_init")
            .ident("init_velocity")
            .ident("init_next_position")
            .ident("food_init");
    });
    script.function("update", [], |s| {
        s.c("The update logic does not run every frame.")
            .ident("frame_count")
            .ident("load")
            .v(5)
            .ident("remainder")
            .ident("return_if_non_zero")
            .c("Looks like it's time to run updates!")
            .ident("should_game_run")
            .ident("load")
            .block(|s| {
                s.ident("handle_input")
                    .ident("update_positions")
                    .ident("food_eat");
            })
            .block(|_| {})
            .ident("if");
    });

    // Game state - should game run
    script.function("init_should_game_run", [], |s| {
        s.v(1).ident("should_game_run").ident("store");
    });

    // Game state - velocity
    script.function("init_velocity", [], |s| {
        s.v(1).v(0).ident("velocity").ident("vec_store");
    });

    // Game state - next position
    script.function("init_next_position", [], |s| {
        s.ident("positions")
            .v(0)
            .ident("vec_buf_get")
            .ident("next_position")
            .ident("vec_store");
    });
    script.function("update_next_position", [], |s| {
        s.ident("snake_head")
            .ident("vec_x")
            .ident("velocity")
            .ident("vec_load")
            .ident("vec_x")
            .ident("add_i8")
            .ident("snake_head")
            .ident("vec_y")
            .ident("velocity")
            .ident("vec_load")
            .ident("vec_y")
            .ident("add_i8")
            .ident("next_position")
            .ident("vec_store")
            .ident("next_position")
            .ident("vec_load")
            .ident("is_out_of_bounds")
            .block(|s| {
                s.ident("next_position")
                    .ident("vec_load")
                    .bind(["next_x", "next_y"])
                    .ident("tile_field_size")
                    .ident("vec_load")
                    .bind(["limit_x", "limit_y"])
                    .ident("next_x")
                    .ident("limit_x")
                    .ident("handle_coordinate_smaller_than_zero")
                    .bind(["next_x"])
                    .ident("next_y")
                    .ident("limit_y")
                    .ident("handle_coordinate_smaller_than_zero")
                    .bind(["next_y"])
                    .ident("next_x")
                    .ident("limit_x")
                    .ident("handle_coordinate_larger_than_limit")
                    .bind(["next_x"])
                    .ident("next_y")
                    .ident("limit_y")
                    .ident("handle_coordinate_larger_than_limit")
                    .bind(["next_y"])
                    .ident("next_x")
                    .ident("next_y")
                    .ident("next_position")
                    .ident("vec_store");
            })
            .block(|_| {})
            .ident("if");
    });
    script.function(
        "handle_coordinate_smaller_than_zero",
        ["coord", "limit"],
        |s| {
            s.v(0)
                .ident("coord")
                .ident("greater")
                .bind(["coord_smaller_than_zero"])
                .ident("coord")
                .ident("coord_smaller_than_zero")
                .block(|s| {
                    s.ident("limit").ident("add_i32");
                })
                .block(|_| {})
                .ident("if");
        },
    );
    script.function(
        "handle_coordinate_larger_than_limit",
        ["coord", "limit"],
        |s| {
            s.ident("limit")
                .ident("coord")
                .ident("greater")
                .bind(["limit_greater_than_coord"])
                .ident("coord")
                .ident("limit_greater_than_coord")
                .ident("return_if_non_zero")
                .ident("limit")
                .ident("sub");
        },
    );

    // Game state - food
    script.function("food_init", [], |s| {
        s.ident("negatable_random")
            .ident("abs")
            .ident("tile_field_size")
            .ident("vec_load")
            .ident("vec_x")
            .ident("remainder")
            .ident("negatable_random")
            .ident("abs")
            .ident("tile_field_size")
            .ident("vec_load")
            .ident("vec_y")
            .ident("remainder")
            .ident("food_position")
            .ident("vec_store");
    });
    script.function("food_eat", [], |s| {
        s.ident("_food_collides_with_snake")
            .block(|s| {
                s.c("The snake's head and the food are at the same position.")
                    .ident("food_init")
                    .ident("grow_snake");
            })
            .block(|_| {})
            .ident("if");
    });
    script.function("_food_collides_with_snake", [], |s| {
        s.ident("snake_head")
            .ident("food_position")
            .ident("vec_load")
            .ident("vec_eq")
            .bind(["head_collides"])
            .ident("food_position")
            .ident("vec_load")
            .ident("check_body_collision")
            .bind(["body_collides"])
            .ident("head_collides")
            .ident("body_collides")
            .ident("add_i32")
            .v(0)
            .ident("greater");
    });

    // Game state - snake
    script.function("snake_init", [], |s| {
        s.v(3)
            .ident("snake_length")
            .ident("store")
            .ident("positions")
            .ident("vec_buf_init")
            .ident("positions")
            .v(15)
            .v(15)
            .ident("vec_buf_push");
    });
    script.function("snake_head", [], |s| {
        s.ident("positions").ident("vec_buf_last");
    });
    script.function("update_positions", [], |s| {
        s.ident("update_next_position")
            .ident("snake_head")
            .ident("check_body_collision")
            .ident("return_if_non_zero")
            .ident("positions")
            .ident("next_position")
            .ident("vec_load")
            .ident("vec_buf_push")
            .ident("pop_positions");
    });
    script.function("pop_positions", [], |s| {
        s.ident("positions")
            .ident("vec_buf_len")
            .ident("snake_length")
            .ident("load")
            .ident("greater")
            .block(|s| {
                s.ident("positions")
                    .ident("vec_buf_pop")
                    .ident("pop_positions");
            })
            .block(|_| {})
            .ident("if");
    });
    script.function("grow_snake", [], |s| {
        s.ident("snake_length")
            .ident("load")
            .v(1)
            .ident("add_i32")
            .bind(["snake_length_plus_growth"])
            .ident("snake_length_plus_growth")
            .ident("positions")
            .ident("vec_buf_capacity")
            .ident("greater")
            .ident("return_if_non_zero")
            .ident("snake_length_plus_growth")
            .ident("snake_length")
            .ident("store");
    });
    script.function("check_body_collision", ["x", "y"], |s| {
        s.ident("x")
            .ident("y")
            .v(0)
            .ident("check_body_collision_inner");
    });
    script.function("check_body_collision_inner", ["x", "y", "index"], |s| {
        s.ident("positions")
            .ident("vec_buf_len")
            .v(1)
            .ident("sub")
            .ident("index")
            .ident("greater")
            .block(|s| {
                s.ident("positions")
                    .ident("index")
                    .ident("vec_buf_get")
                    .ident("vec_x")
                    .ident("x")
                    .ident("eq")
                    .bind(["x_matches"])
                    .ident("positions")
                    .ident("index")
                    .ident("vec_buf_get")
                    .ident("vec_y")
                    .ident("y")
                    .ident("eq")
                    .bind(["y_matches"])
                    .ident("x_matches")
                    .ident("y_matches")
                    .ident("add_i32")
                    .v(2)
                    .ident("eq")
                    .ident("copy")
                    .ident("return_if_non_zero")
                    .ident("drop")
                    .ident("x")
                    .ident("y")
                    .ident("index")
                    .v(1)
                    .ident("add_i32")
                    .ident("check_body_collision_inner");
            })
            .block(|s| {
                s.v(0);
            })
            .ident("if");
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
            .ident("read_input")
            .bind(["input"])
            .c("Return, if no input is available.")
            .ident("input")
            .ident("return_if_zero")
            .c("Assume result was `1`, and apply an `up` event.")
            .v(0)
            .v(-1)
            .ident("i32_to_i8")
            .ident("velocity")
            .ident("vec_store")
            .c("Now check if it actually was an `up` event, and if so, return.")
            .ident("input")
            .v(1)
            .ident("sub")
            .ident("copy")
            .ident("return_if_zero")
            .c("Seems it wasn't `up`. Try again for `left`.")
            .v(-1)
            .ident("i32_to_i8")
            .v(0)
            .ident("velocity")
            .ident("vec_store")
            .ident("input")
            .v(2)
            .ident("sub")
            .ident("copy")
            .ident("return_if_zero")
            .c("It wasn't `left` either. Re-try for `down`.")
            .v(0)
            .v(1)
            .ident("velocity")
            .ident("vec_store")
            .ident("input")
            .v(3)
            .ident("sub")
            .ident("copy")
            .ident("return_if_zero")
            .c("Guessed wrong again. One more try for `right`.")
            .v(1)
            .v(0)
            .ident("velocity")
            .ident("vec_store")
            .ident("input")
            .v(4)
            .ident("sub")
            .ident("copy")
            .ident("return_if_zero")
            .c("It wasn't `right` either, which means `read_input` returned")
            .c("an unexpected value.")
            .ident("brk");
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
        s.ident("x");
    });
    script.function("vec_y", ["_", "y"], |s| {
        s.ident("y");
    });
    script.function("vec_load", ["address"], |s| {
        s.ident("address")
            .ident("load")
            .ident("address")
            .v(1)
            .ident("add_i32")
            .ident("load");
    });
    script.function("vec_store", ["x", "y", "address"], |s| {
        s.ident("x")
            .ident("address")
            .ident("store")
            .ident("y")
            .ident("address")
            .v(1)
            .ident("add_i32")
            .ident("store");
    });
    script.function("vec_copy", ["vx", "vy"], |s| {
        s.ident("vx").ident("vy").ident("vx").ident("vy");
    });
    script.function("vec_drop", ["_", "_"], |_| {});
    script.function("vec_eq", ["ax", "ay", "bx", "by"], |s| {
        s.ident("ax")
            .ident("bx")
            .ident("eq")
            .ident("copy")
            .ident("return_if_zero")
            .ident("drop")
            .ident("ay")
            .ident("by")
            .ident("eq")
            .ident("copy")
            .ident("return_if_zero")
            .ident("drop")
            .c("Vectors are equal!")
            .v(1);
    });

    // Utilities - Vector Buffer
    script.function("vec_buf_init", ["vec_buf"], |s| {
        s.v(0)
            .ident("vec_buf")
            .ident("_vec_buf_first")
            .ident("store")
            .v(0)
            .ident("vec_buf")
            .ident("_vec_buf_next")
            .ident("store")
            .v(64)
            .ident("vec_buf")
            .ident("_vec_buf_capacity")
            .ident("store");
    });
    script.function("vec_buf_get", ["vec_buf", "index"], |s| {
        s.ident("index")
            .v(2)
            .ident("mul_i32")
            .bind(["offset"])
            .ident("vec_buf")
            .ident("_vec_buf_first")
            .ident("load")
            .bind(["base"])
            .ident("vec_buf")
            .ident("base")
            .ident("offset")
            .ident("_vec_buf_address")
            .ident("vec_load");
    });
    script.function("vec_buf_last", ["vec_buf"], |s| {
        s.ident("vec_buf")
            .ident("vec_buf_len")
            .v(1)
            .ident("sub")
            .bind(["index"])
            .ident("vec_buf")
            .ident("index")
            .ident("vec_buf_get");
    });
    script.function("vec_buf_push", ["vec_buf", "x", "y"], |s| {
        s.ident("vec_buf")
            .ident("_vec_buf_next")
            .bind(["next_addr"])
            .ident("vec_buf")
            .ident("next_addr")
            .ident("load")
            .v(0)
            .ident("_vec_buf_address")
            .bind(["address"])
            .ident("x")
            .ident("y")
            .ident("address")
            .ident("vec_store")
            .ident("next_addr")
            .ident("_vec_buf_inc_index");
    });
    script.function("vec_buf_pop", ["vec_buf"], |s| {
        s.ident("vec_buf")
            .ident("_vec_buf_first")
            .ident("_vec_buf_inc_index");
    });
    script.function("vec_buf_len", ["vec_buf"], |s| {
        s.ident("vec_buf")
            .ident("_vec_buf_first")
            .ident("load")
            .bind(["first"])
            .ident("vec_buf")
            .ident("_vec_buf_next")
            .ident("load")
            .bind(["next"])
            .ident("next")
            .ident("first")
            .ident("sub")
            .v(2)
            .ident("div")
            .bind(["difference"])
            .ident("difference")
            .ident("difference")
            .ident("return_if_zero")
            .v(0)
            .ident("difference")
            .ident("greater")
            .ident("return_if_zero")
            .ident("vec_buf")
            .ident("_vec_buf_capacity")
            .ident("load")
            .ident("add_i32");
    });
    script.function("vec_buf_capacity", ["vec_buf"], |s| {
        s.ident("vec_buf")
            .ident("_vec_buf_capacity")
            .ident("load")
            .v(2)
            .ident("div");
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
            .ident("base")
            .ident("offset")
            .ident("add_u8_wrap")
            .ident("vec_buf")
            .ident("_vec_buf_capacity")
            .ident("load")
            .ident("remainder")
            .ident("vec_buf")
            .ident("_vec_buf_buffer")
            .ident("add_u8_wrap");
    });
    script.function("_vec_buf_inc_index", ["index_addr"], |s| {
        s.ident("index_addr")
            .ident("load")
            .v(2)
            .ident("add_u8_wrap")
            .ident("index_addr")
            .ident("store");
    });
    script.function("_vec_buf_first", ["vec_buf"], |s| {
        s.ident("vec_buf").v(0).ident("add_i32");
    });
    script.function("_vec_buf_next", ["vec_buf"], |s| {
        s.ident("vec_buf").v(1).ident("add_i32");
    });
    script.function("_vec_buf_capacity", ["vec_buf"], |s| {
        s.ident("vec_buf").v(2).ident("add_i32");
    });
    script.function("_vec_buf_buffer", ["vec_buf"], |s| {
        s.ident("vec_buf").v(3).ident("add_i32");
    });

    // Utilities - Miscellaneous
    script.function("negatable_random", [], |s| {
        s.c("Negating the minimum number would result in an integer overflow.")
            .ident("read_random")
            .ident("copy")
            .ident("word_min")
            .ident("eq")
            .ident("return_if_zero")
            .ident("drop")
            .c("Looks like we ran into the minimum. Try again!")
            .ident("negatable_random");
    });
    script.function("abs", ["v"], |s| {
        s.ident("v")
            .ident("v")
            .v(-1)
            .ident("greater")
            .ident("return_if_non_zero")
            .ident("neg");
    });

    // Utilities - Words
    script.function("word_min", [], |s| {
        s.v(i32::MIN);
    });
}
