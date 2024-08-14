use capi_compiler::syntax::Script;

pub fn main() {
    let mut script = Script::default();
    snake(&mut script);

    let script = ron::to_string(&script).unwrap();
    println!("{script}");
}

fn snake(script: &mut Script) {
    // Main loop
    script.function(|b| {
        b.branch(
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
        )
    });
    script.function(|b| {
        b.branch(
            "main_inner",
            |p| p,
            |s| {
                s.ident("draw")
                    .ident("count_frame")
                    .ident("update")
                    .ident("main_inner");
            },
        )
    });

    // Draw
    script.function(|b| {
        b.branch(
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
        )
    });
    script
        .function(|b| {
            b.branch(
                "draw_snake",
                |p| p,
                |s| {
                    s.v(0).ident("_draw_snake_inner");
                },
            )
        })
        .function(|b| {
            b.branch(
                "_draw_snake_inner",
                |p| p.ident("index"),
                |e| {
                    e.ident("positions")
                        .ident("vec_buf_len")
                        .ident("index")
                        .ident("greater_i8")
                        .bind(["index_is_within_bounds"])
                        .ident("index_is_within_bounds")
                        .ident("index")
                        .ident("_draw_snake_draw_rest_of_snake");
                },
            )
        })
        .function(|b| {
            b.branch(
                "_draw_snake_draw_rest_of_snake",
                |p| p.lit(0).ident("_"),
                |e| {
                    e.c("Index is out of bounds. We're done");
                },
            )
        })
        .function(|b| {
            b.branch(
                "_draw_snake_draw_rest_of_snake",
                |p| p.lit(1).ident("index"),
                |e| {
                    e.c("Index is valid. Continue drawing the snake.")
                        .ident("index")
                        .ident("_draw_snake_draw_body_segment")
                        .ident("index")
                        .v(1)
                        .ident("add_u8")
                        .ident("_draw_snake_inner");
                },
            )
        })
        .function(|b| {
            b.branch(
                "_draw_snake_draw_body_segment",
                |p| p.ident("index"),
                |e| {
                    e.ident("positions")
                        .ident("index")
                        .ident("vec_buf_get")
                        .ident("_draw_snake_body_color")
                        .ident("set_pixel");
                },
            )
        })
        .function(|b| {
            b.branch(
                "_draw_snake_body_color",
                |p| p,
                |e| {
                    e.v(0).v(255).v(0).v(255);
                },
            )
        });
    script.function(|b| {
        b.branch(
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
        )
    });

    // Draw - clear pixels
    script.function(|b| {
        b.branch(
            "clear_pixels",
            |p| p,
            |s| {
                s.ident("init_tile_index").ident("clear_pixels_inner");
            },
        )
    });
    script.function(|b| {
        b.branch(
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
        )
    });

    // Draw - write tiles - tile index
    script.function(|b| {
        b.branch(
            "init_tile_index",
            |p| p,
            |s| {
                s.v(0).v(0);
            },
        )
    });
    script.function(|b| {
        b.branch(
            "check_tile_index",
            |p| p.ident("tile_y"),
            |s| {
                s.ident("tile_field_size")
            .ident("vec_load")
            .ident("vec_y")
            .ident("tile_y")
            .c("Leave zero, if the y-coordinate has advanced beyond the last")
            .c("line of the tile field. Otherwise, leave non-zero value.")
            .ident("sub_i32");
            },
        )
    });
    script
        .function(|b| {
            b.branch(
                "increment_tile_index",
                |p| p.ident("tile_x").ident("tile_y"),
                |s| {
                    s.ident("tile_x")
                        .ident("_increment_tile_index_increment_coord")
                        .ident("_increment_tile_index_is_tile_x_within_limit")
                        .ident("_increment_tile_index_reset_x_if_overflowed")
                        .ident("tile_y")
                        .ident(
                            "_increment_tile_index_increment_y_if_necessary",
                        );
                },
            )
        })
        .function(|b| {
            b.branch(
                "_increment_tile_index_increment_coord",
                |p| p.ident("coord"),
                |e| {
                    e.ident("coord").v(1).ident("add_u8");
                },
            )
        })
        .function(|b| {
            b.branch(
                "_increment_tile_index_is_tile_x_within_limit",
                |p| p.ident("tile_x"),
                |e| {
                    e.ident("tile_field_size")
                        .ident("vec_load")
                        .ident("vec_x")
                        .ident("tile_x")
                        .ident("greater_u8")
                        .bind(["tile_x_within_limit"])
                        .ident("tile_x")
                        .ident("tile_x_within_limit");
                },
            )
        })
        .function(|b| {
            b.branch(
                "_increment_tile_index_reset_x_if_overflowed",
                |p| p.ident("_").lit(0),
                |e| {
                    e.v(0).v(0);
                },
            )
        })
        .function(|b| {
            b.branch(
                "_increment_tile_index_reset_x_if_overflowed",
                |p| p.ident("tile_x").lit(1),
                |e| {
                    e.ident("tile_x").v(1);
                },
            )
        })
        .function(|b| {
            b.branch(
                "_increment_tile_index_increment_y_if_necessary",
                |p| p.ident("tile_x").lit(0).ident("tile_y"),
                |e| {
                    e.ident("tile_x")
                        .ident("tile_y")
                        .ident("_increment_tile_index_increment_coord");
                },
            )
        })
        .function(|b| {
            b.branch(
                "_increment_tile_index_increment_y_if_necessary",
                |p| p.ident("tile_x").lit(1).ident("tile_y"),
                |e| {
                    e.ident("tile_x").ident("tile_y");
                },
            )
        });

    // Tile field size
    script
        .function(|b| {
            b.branch(
                "is_out_of_bounds",
                |p| p.ident("x").ident("y"),
                |s| {
                    s.ident("tile_field_size")
                        .ident("vec_load")
                        .bind(["limit_x", "limit_y"])
                        .ident("x")
                        .ident("limit_x")
                        .ident("_is_out_of_bounds_is_coord_within_bounds")
                        .ident("y")
                        .ident("limit_y")
                        .ident("_is_out_of_bounds_is_coord_within_bounds")
                        .ident("and")
                        .ident("not");
                },
            )
        })
        .function(|b| {
            b.branch(
                "_is_out_of_bounds_is_coord_within_bounds",
                |p| p.ident("coord").ident("limit"),
                |e| {
                    e.ident("coord")
                        .v(0)
                        .ident("greater_u8")
                        .ident("limit")
                        .ident("coord")
                        .ident("greater_u8")
                        .ident("and");
                },
            )
        });

    // Frame count
    script.function(|b| {
        b.branch(
            "init_frame_count",
            |p| p,
            |s| {
                s.v(1).ident("frame_count").ident("store");
            },
        )
    });
    script
        .function(|b| {
            b.branch(
                "count_frame",
                |p| p,
                |s| {
                    s.c("Grab the current frame count.")
                    .ident("frame_count")
                    .ident("load")
                    .c("Increment the frame count.")
                    .v(1)
                    .ident("add_i32")
                    .c("Place a copy of the new frame count back where it came")
                    .c("from.")
                    .ident("copy")
                    .ident("frame_count")
                    .ident("store")
                    .ident("_count_frame_reset_frame_count_if_necessary");
                },
            )
        })
        .function(|b| {
            b.branch(
                "_count_frame_reset_frame_count_if_necessary",
                |p| p.lit(121),
                |e| {
                    e.ident("init_frame_count");
                },
            )
        })
        .function(|b| {
            b.branch(
                "_count_frame_reset_frame_count_if_necessary",
                |p| p.ident("_"),
                |_| {},
            )
        });

    // Game state
    script.function(|b| {
        b.branch(
            "init",
            |p| p,
            |s| {
                s.ident("init_should_game_run")
                    .ident("snake_init")
                    .ident("init_velocity")
                    .ident("init_next_position")
                    .ident("food_init");
            },
        )
    });
    script.function(|b| {
        b.branch(
            "update",
            |p| p,
            |s| {
                s.c("The update logic does not run every frame.")
                    .ident("frame_count")
                    .ident("load")
                    .v(5)
                    .ident("remainder_i32")
                    .ident("not")
                    .ident("should_game_run")
                    .ident("load")
                    .ident("and")
                    .block(|s| {
                        s.ident("read_input")
                            .ident("handle_input")
                            .ident("update_positions")
                            .ident("food_eat");
                    })
                    .block(|_| {})
                    .ident("if");
            },
        )
    });

    // Game state - should game run
    script.function(|b| {
        b.branch(
            "init_should_game_run",
            |p| p,
            |s| {
                s.v(1).ident("should_game_run").ident("store");
            },
        )
    });

    // Game state - velocity
    script.function(|b| {
        b.branch(
            "init_velocity",
            |p| p,
            |s| {
                s.v(1).v(0).ident("velocity").ident("vec_store");
            },
        )
    });

    // Game state - next position
    script.function(|b| {
        b.branch(
            "init_next_position",
            |p| p,
            |s| {
                s.ident("positions")
                    .v(0)
                    .ident("vec_buf_get")
                    .ident("next_position")
                    .ident("vec_store");
            },
        )
    });
    script.function(|b| {
        b.branch(
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
        )
    });
    script.function(|b| {
        b.branch(
            "handle_coordinate_smaller_than_zero",
            |p| p.ident("coord").ident("limit"),
            |s| {
                s.v(0)
                    .ident("coord")
                    .ident("greater_i8")
                    .bind(["coord_smaller_than_zero"])
                    .ident("coord_smaller_than_zero")
                    .block(|s| {
                        s.ident("coord").ident("limit").ident("add_i8");
                    })
                    .block(|e| {
                        e.ident("coord");
                    })
                    .ident("if");
            },
        )
    });
    script.function(|b| {
        b.branch(
            "handle_coordinate_larger_than_limit",
            |p| p.ident("coord").ident("limit"),
            |s| {
                s.ident("limit")
                    .ident("coord")
                    .ident("greater_i32")
                    .bind(["limit_greater_than_coord"])
                    .ident("coord")
                    .ident("limit_greater_than_coord")
                    .ident("return_if_non_zero")
                    .ident("limit")
                    .ident("sub_i32");
            },
        )
    });

    // Game state - food
    script.function(|b| {
        b.branch(
            "food_init",
            |p| p,
            |s| {
                s.ident("negatable_random")
                    .ident("abs")
                    .ident("tile_field_size")
                    .ident("vec_load")
                    .ident("vec_x")
                    .ident("remainder_i32")
                    .ident("negatable_random")
                    .ident("abs")
                    .ident("tile_field_size")
                    .ident("vec_load")
                    .ident("vec_y")
                    .ident("remainder_i32")
                    .ident("food_position")
                    .ident("vec_store");
            },
        )
    });
    script.function(|b| {
        b.branch(
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
        )
    });
    script.function(|b| {
        b.branch(
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
                    .ident("greater_i32");
            },
        )
    });

    // Game state - snake
    script.function(|b| {
        b.branch(
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
        )
    });
    script.function(|b| {
        b.branch(
            "snake_head",
            |p| p,
            |s| {
                s.ident("positions").ident("vec_buf_last");
            },
        )
    });
    script.function(|b| {
        b.branch(
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
        )
    });
    script.function(|b| {
        b.branch(
            "pop_positions",
            |p| p,
            |s| {
                s.ident("positions")
                    .ident("vec_buf_len")
                    .ident("snake_length")
                    .ident("load")
                    .ident("greater_i32")
                    .block(|s| {
                        s.ident("positions")
                            .ident("vec_buf_pop")
                            .ident("pop_positions");
                    })
                    .block(|_| {})
                    .ident("if");
            },
        )
    });
    script.function(|b| {
        b.branch(
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
                    .ident("greater_i32")
                    .ident("return_if_non_zero")
                    .ident("snake_length_plus_growth")
                    .ident("snake_length")
                    .ident("store");
            },
        )
    });
    script.function(|b| {
        b.branch(
            "check_body_collision",
            |p| p.ident("x").ident("y"),
            |s| {
                s.ident("x")
                    .ident("y")
                    .v(0)
                    .ident("check_body_collision_inner");
            },
        )
    });
    script.function(|b| {
        b.branch(
            "check_body_collision_inner",
            |p| p.ident("x").ident("y").ident("index"),
            |s| {
                s.ident("positions")
                    .ident("vec_buf_len")
                    .v(1)
                    .ident("sub_i32")
                    .ident("index")
                    .ident("greater_i32")
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
        )
    });

    // Input
    script
        .function(|b| {
            b.branch(
                "handle_input",
                |p| p.lit(0),
                |e| {
                    e.c("No input available.");
                },
            )
        })
        .function(|b| {
            b.branch(
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
        })
        .function(|b| {
            b.branch(
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
        })
        .function(|b| {
            b.branch(
                "handle_input",
                |p| p.lit(3),
                |e| {
                    e.c("down").v(0).v(1).ident("velocity").ident("vec_store");
                },
            )
        })
        .function(|b| {
            b.branch(
                "handle_input",
                |p| p.lit(4),
                |e| {
                    e.c("right").v(1).v(0).ident("velocity").ident("vec_store");
                },
            )
        })
        .function(|b| {
            b.branch(
                "handle_input",
                |p| p.ident("_"),
                |e| {
                    e.c("unexpected value").ident("brk");
                },
            )
        });

    // Memory map
    script.function(|b| {
        b.branch(
            "tile_field_size",
            |p| p,
            |s| {
                s.v(0);
            },
        )
    });
    script.function(|b| {
        b.branch(
            "frame_count",
            |p| p,
            |s| {
                s.v(2);
            },
        )
    });
    script.function(|b| {
        b.branch(
            "should_game_run",
            |p| p,
            |s| {
                s.v(3);
            },
        )
    });
    script.function(|b| {
        b.branch(
            "velocity",
            |p| p,
            |s| {
                s.v(4);
            },
        )
    });
    script.function(|b| {
        b.branch(
            "next_position",
            |p| p,
            |s| {
                s.v(6);
            },
        )
    });
    script.function(|b| {
        b.branch(
            "food_position",
            |p| p,
            |s| {
                s.v(8);
            },
        )
    });
    script.function(|b| {
        b.branch(
            "snake_length",
            |p| p,
            |s| {
                s.v(10);
            },
        )
    });
    script.function(|b| {
        b.branch(
            "positions",
            |p| p,
            |s| {
                s.v(11);
            },
        )
    });

    // Utilities - Vector
    script.function(|b| {
        b.branch(
            "vec_x",
            |p| p.ident("x").ident("_"),
            |s| {
                s.ident("x");
            },
        )
    });
    script.function(|b| {
        b.branch(
            "vec_y",
            |p| p.ident("_").ident("y"),
            |s| {
                s.ident("y");
            },
        )
    });
    script.function(|b| {
        b.branch(
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
        )
    });
    script.function(|b| {
        b.branch(
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
        )
    });
    script.function(|b| {
        b.branch(
            "vec_copy",
            |p| p.ident("vx").ident("vy"),
            |s| {
                s.ident("vx").ident("vy").ident("vx").ident("vy");
            },
        )
    });
    script.function(|b| {
        b.branch("vec_drop", |p| p.ident("_").ident("_"), |_| {})
    });
    script.function(|b| {
        b.branch(
            "vec_eq",
            |p| p.ident("ax").ident("ay").ident("bx").ident("by"),
            |s| {
                s.ident("ax")
                    .ident("bx")
                    .ident("eq")
                    .ident("ay")
                    .ident("by")
                    .ident("eq")
                    .ident("and");
            },
        )
    });

    // Utilities - Vector Buffer
    script.function(|b| {
        b.branch(
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
        )
    });
    script.function(|b| {
        b.branch(
            "vec_buf_get",
            |p| p.ident("vec_buf").ident("index"),
            |s| {
                s.ident("index")
                    .v(2)
                    .ident("mul_u8_wrap")
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
        )
    });
    script.function(|b| {
        b.branch(
            "vec_buf_last",
            |p| p.ident("vec_buf"),
            |s| {
                s.ident("vec_buf")
                    .ident("vec_buf_len")
                    .v(1)
                    .ident("sub_u8")
                    .bind(["index"])
                    .ident("vec_buf")
                    .ident("index")
                    .ident("vec_buf_get");
            },
        )
    });
    script.function(|b| {
        b.branch(
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
        )
    });
    script.function(|b| {
        b.branch(
            "vec_buf_pop",
            |p| p.ident("vec_buf"),
            |s| {
                s.ident("vec_buf")
                    .ident("_vec_buf_first")
                    .ident("_vec_buf_inc_index");
            },
        )
    });
    script.function(|b| {
        b.branch(
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
                    .ident("sub_u8_wrap")
                    .v(2)
                    .ident("div_u8");
            },
        )
    });
    script.function(|b| {
        b.branch(
            "vec_buf_capacity",
            |p| p.ident("vec_buf"),
            |s| {
                s.ident("vec_buf")
                    .ident("_vec_buf_capacity")
                    .ident("load")
                    .v(2)
                    .ident("div_i32");
            },
        )
    });
    script.function(|b| {
        b.branch(
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
            .ident("remainder_i32")
            .ident("vec_buf")
            .ident("_vec_buf_buffer")
            .ident("add_u8_wrap");
        },
    )
    });
    script.function(|b| {
        b.branch(
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
        )
    });
    script.function(|b| {
        b.branch(
            "_vec_buf_first",
            |p| p.ident("vec_buf"),
            |s| {
                s.ident("vec_buf").v(0).ident("add_i32");
            },
        )
    });
    script.function(|b| {
        b.branch(
            "_vec_buf_next",
            |p| p.ident("vec_buf"),
            |s| {
                s.ident("vec_buf").v(1).ident("add_i32");
            },
        )
    });
    script.function(|b| {
        b.branch(
            "_vec_buf_capacity",
            |p| p.ident("vec_buf"),
            |s| {
                s.ident("vec_buf").v(2).ident("add_i32");
            },
        )
    });
    script.function(|b| {
        b.branch(
            "_vec_buf_buffer",
            |p| p.ident("vec_buf"),
            |s| {
                s.ident("vec_buf").v(3).ident("add_i32");
            },
        )
    });

    // Utilities - Miscellaneous
    script
        .function(|b| {
            b.branch(
                "negatable_random",
                |p| p,
                |s| {
                    s.ident("read_random")
                        .ident("_negatable_random_return_or_continue");
                },
            )
        })
        .function(|b| {
            b.branch(
            "_negatable_random_return_or_continue",
            |p| p.lit(i32::MIN),
            |e| {
                e.c("Negating the minimum number would result in an integer")
                    .c("overflow.")
                    .ident("negatable_random");
            },
            )
        })
        .function(|b| {
            b.branch(
                "_negatable_random_return_or_continue",
                |p| p.ident("random"),
                |e| {
                    e.ident("random");
                },
            )
        });
    script.function(|b| {
        b.branch(
            "abs",
            |p| p.ident("v"),
            |s| {
                s.ident("v")
                    .ident("v")
                    .v(-1)
                    .ident("greater_i32")
                    .ident("return_if_non_zero")
                    .ident("neg_i32");
            },
        )
    });
}
