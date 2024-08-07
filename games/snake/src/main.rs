use capi_compiler::syntax::Script;

pub fn main() {
    let mut script = Script::default();
    snake(&mut script);

    let script = ron::to_string(&script).unwrap();
    println!("{script}");
}

fn snake(script: &mut Script) {
    // Main loop
    script.function(
        "main",
        |p| p.ident("size_x").ident("size_y"),
        |s| {
            s.ident("size_x")
                .ident("size_y")
                .ident("tile_field_size")
                .ident("vec_store")
                .ident("init_frame_count")
                .ident("init")
                .ident("main_inner");
        },
    );
    script.function(
        "main_inner",
        |p| p,
        |s| {
            s.ident("draw")
                .ident("count_frame")
                .ident("update")
                .ident("main_inner");
        },
    );

    // Draw
    script.function(
        "draw",
        |p| p,
        |s| {
            s.ident("clear_pixels")
            .ident("draw_snake")
            .ident("draw_food")
            .c("This blocks until the display system is ready to process the")
            .c("next frame.")
            .ident("submit_frame");
        },
    );
    script.function(
        "draw_snake",
        |p| p,
        |s| {
            s.v(0).ident("draw_snake_inner");
        },
    );
    script.function(
        "draw_snake_inner",
        |p| p.ident("index"),
        |s| {
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
        },
    );
    script.function(
        "draw_food",
        |p| p,
        |s| {
            s.ident("food_position")
                .ident("vec_load")
                .v(255)
                .v(0)
                .v(0)
                .v(255)
                .ident("set_pixel");
        },
    );

    // Draw - clear pixels
    script.function(
        "clear_pixels",
        |p| p,
        |s| {
            s.ident("init_tile_index").ident("clear_pixels_inner");
        },
    );
    script.function(
        "clear_pixels_inner",
        |p| p.ident("tile_x").ident("tile_y"),
        |s| {
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
        },
    );

    // Draw - write tiles - tile index
    script.function(
        "init_tile_index",
        |p| p,
        |s| {
            s.v(0).v(0);
        },
    );
    script.function(
        "check_tile_index",
        |p| p.ident("tile_y"),
        |s| {
            s.ident("tile_field_size")
            .ident("vec_load")
            .ident("vec_y")
            .ident("tile_y")
            .c("Leave zero, if the y-coordinate has advanced beyond the last")
            .c("line of the tile field. Otherwise, leave non-zero value.")
            .ident("sub");
        },
    );
    script.function(
        "increment_tile_index",
        |p| p.ident("tile_x").ident("tile_y"),
        |s| {
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
        },
    );

    // Tile field size
    script.function(
        "is_out_of_bounds",
        |p| p.ident("x").ident("y"),
        |s| {
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
        },
    );

    // Frame count
    script.function(
        "init_frame_count",
        |p| p,
        |s| {
            s.v(1).ident("frame_count").ident("store");
        },
    );
    script.function(
        "count_frame",
        |p| p,
        |s| {
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
        },
    );

    // Game state
    script.function(
        "init",
        |p| p,
        |s| {
            s.ident("init_should_game_run")
                .ident("snake_init")
                .ident("init_velocity")
                .ident("init_next_position")
                .ident("food_init");
        },
    );
    script.function(
        "update",
        |p| p,
        |s| {
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
                    s.ident("read_input")
                        .ident("handle_input")
                        .ident("update_positions")
                        .ident("food_eat");
                })
                .block(|_| {})
                .ident("if");
        },
    );

    // Game state - should game run
    script.function(
        "init_should_game_run",
        |p| p,
        |s| {
            s.v(1).ident("should_game_run").ident("store");
        },
    );

    // Game state - velocity
    script.function(
        "init_velocity",
        |p| p,
        |s| {
            s.v(1).v(0).ident("velocity").ident("vec_store");
        },
    );

    // Game state - next position
    script.function(
        "init_next_position",
        |p| p,
        |s| {
            s.ident("positions")
                .v(0)
                .ident("vec_buf_get")
                .ident("next_position")
                .ident("vec_store");
        },
    );
    script.function(
        "update_next_position",
        |p| p,
        |s| {
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
        },
    );
    script.function(
        "handle_coordinate_smaller_than_zero",
        |p| p.ident("coord").ident("limit"),
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
        |p| p.ident("coord").ident("limit"),
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
    script.function(
        "food_init",
        |p| p,
        |s| {
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
        },
    );
    script.function(
        "food_eat",
        |p| p,
        |s| {
            s.ident("_food_collides_with_snake")
            .block(|s| {
                s.c("The snake's head and the food are at the same position.")
                    .ident("food_init")
                    .ident("grow_snake");
            })
            .block(|_| {})
            .ident("if");
        },
    );
    script.function(
        "_food_collides_with_snake",
        |p| p,
        |s| {
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
        },
    );

    // Game state - snake
    script.function(
        "snake_init",
        |p| p,
        |s| {
            s.v(3)
                .ident("snake_length")
                .ident("store")
                .ident("positions")
                .ident("vec_buf_init")
                .ident("positions")
                .v(15)
                .v(15)
                .ident("vec_buf_push");
        },
    );
    script.function(
        "snake_head",
        |p| p,
        |s| {
            s.ident("positions").ident("vec_buf_last");
        },
    );
    script.function(
        "update_positions",
        |p| p,
        |s| {
            s.ident("update_next_position")
                .ident("snake_head")
                .ident("check_body_collision")
                .ident("return_if_non_zero")
                .ident("positions")
                .ident("next_position")
                .ident("vec_load")
                .ident("vec_buf_push")
                .ident("pop_positions");
        },
    );
    script.function(
        "pop_positions",
        |p| p,
        |s| {
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
        },
    );
    script.function(
        "grow_snake",
        |p| p,
        |s| {
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
        },
    );
    script.function(
        "check_body_collision",
        |p| p.ident("x").ident("y"),
        |s| {
            s.ident("x")
                .ident("y")
                .v(0)
                .ident("check_body_collision_inner");
        },
    );
    script.function(
        "check_body_collision_inner",
        |p| p.ident("x").ident("y").ident("index"),
        |s| {
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
        },
    );

    // Input
    script
        .function(
            "handle_input",
            |p| p.lit(0),
            |e| {
                e.c("No input available.");
            },
        )
        .function(
            "handle_input",
            |p| p.lit(1),
            |e| {
                e.c("up")
                    .v(0)
                    .v(-1)
                    .ident("i32_to_i8")
                    .ident("velocity")
                    .ident("vec_store");
            },
        )
        .function(
            "handle_input",
            |p| p.lit(2),
            |e| {
                e.c("left")
                    .v(-1)
                    .ident("i32_to_i8")
                    .v(0)
                    .ident("velocity")
                    .ident("vec_store");
            },
        )
        .function(
            "handle_input",
            |p| p.lit(3),
            |e| {
                e.c("down").v(0).v(1).ident("velocity").ident("vec_store");
            },
        )
        .function(
            "handle_input",
            |p| p.lit(4),
            |e| {
                e.c("right").v(1).v(0).ident("velocity").ident("vec_store");
            },
        )
        .function(
            "handle_input",
            |p| p.ident("_"),
            |e| {
                e.c("unexpected value").ident("brk");
            },
        );

    // Memory map
    script.function(
        "tile_field_size",
        |p| p,
        |s| {
            s.v(0);
        },
    );
    script.function(
        "frame_count",
        |p| p,
        |s| {
            s.v(2);
        },
    );
    script.function(
        "should_game_run",
        |p| p,
        |s| {
            s.v(3);
        },
    );
    script.function(
        "velocity",
        |p| p,
        |s| {
            s.v(4);
        },
    );
    script.function(
        "next_position",
        |p| p,
        |s| {
            s.v(6);
        },
    );
    script.function(
        "food_position",
        |p| p,
        |s| {
            s.v(8);
        },
    );
    script.function(
        "snake_length",
        |p| p,
        |s| {
            s.v(10);
        },
    );
    script.function(
        "positions",
        |p| p,
        |s| {
            s.v(11);
        },
    );

    // Utilities - Vector
    script.function(
        "vec_x",
        |p| p.ident("x").ident("_"),
        |s| {
            s.ident("x");
        },
    );
    script.function(
        "vec_y",
        |p| p.ident("_").ident("y"),
        |s| {
            s.ident("y");
        },
    );
    script.function(
        "vec_load",
        |p| p.ident("address"),
        |s| {
            s.ident("address")
                .ident("load")
                .ident("address")
                .v(1)
                .ident("add_i32")
                .ident("load");
        },
    );
    script.function(
        "vec_store",
        |p| p.ident("x").ident("y").ident("address"),
        |s| {
            s.ident("x")
                .ident("address")
                .ident("store")
                .ident("y")
                .ident("address")
                .v(1)
                .ident("add_i32")
                .ident("store");
        },
    );
    script.function(
        "vec_copy",
        |p| p.ident("vx").ident("vy"),
        |s| {
            s.ident("vx").ident("vy").ident("vx").ident("vy");
        },
    );
    script.function("vec_drop", |p| p.ident("_").ident("_"), |_| {});
    script.function(
        "vec_eq",
        |p| p.ident("ax").ident("ay").ident("bx").ident("by"),
        |s| {
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
        },
    );

    // Utilities - Vector Buffer
    script.function(
        "vec_buf_init",
        |p| p.ident("vec_buf"),
        |s| {
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
        },
    );
    script.function(
        "vec_buf_get",
        |p| p.ident("vec_buf").ident("index"),
        |s| {
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
        },
    );
    script.function(
        "vec_buf_last",
        |p| p.ident("vec_buf"),
        |s| {
            s.ident("vec_buf")
                .ident("vec_buf_len")
                .v(1)
                .ident("sub")
                .bind(["index"])
                .ident("vec_buf")
                .ident("index")
                .ident("vec_buf_get");
        },
    );
    script.function(
        "vec_buf_push",
        |p| p.ident("vec_buf").ident("x").ident("y"),
        |s| {
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
        },
    );
    script.function(
        "vec_buf_pop",
        |p| p.ident("vec_buf"),
        |s| {
            s.ident("vec_buf")
                .ident("_vec_buf_first")
                .ident("_vec_buf_inc_index");
        },
    );
    script.function(
        "vec_buf_len",
        |p| p.ident("vec_buf"),
        |s| {
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
        },
    );
    script.function(
        "vec_buf_capacity",
        |p| p.ident("vec_buf"),
        |s| {
            s.ident("vec_buf")
                .ident("_vec_buf_capacity")
                .ident("load")
                .v(2)
                .ident("div");
        },
    );
    script.function(
        "_vec_buf_address",
        |p| p.ident("vec_buf").ident("base").ident("offset"),
        |s| {
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
        },
    );
    script.function(
        "_vec_buf_inc_index",
        |p| p.ident("index_addr"),
        |s| {
            s.ident("index_addr")
                .ident("load")
                .v(2)
                .ident("add_u8_wrap")
                .ident("index_addr")
                .ident("store");
        },
    );
    script.function(
        "_vec_buf_first",
        |p| p.ident("vec_buf"),
        |s| {
            s.ident("vec_buf").v(0).ident("add_i32");
        },
    );
    script.function(
        "_vec_buf_next",
        |p| p.ident("vec_buf"),
        |s| {
            s.ident("vec_buf").v(1).ident("add_i32");
        },
    );
    script.function(
        "_vec_buf_capacity",
        |p| p.ident("vec_buf"),
        |s| {
            s.ident("vec_buf").v(2).ident("add_i32");
        },
    );
    script.function(
        "_vec_buf_buffer",
        |p| p.ident("vec_buf"),
        |s| {
            s.ident("vec_buf").v(3).ident("add_i32");
        },
    );

    // Utilities - Miscellaneous
    script.function("negatable_random", |p| p, |s| {
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
    script.function(
        "abs",
        |p| p.ident("v"),
        |s| {
            s.ident("v")
                .ident("v")
                .v(-1)
                .ident("greater")
                .ident("return_if_non_zero")
                .ident("neg");
        },
    );

    // Utilities - Words
    script.function(
        "word_min",
        |p| p,
        |s| {
            s.v(i32::MIN);
        },
    );
}
