use std::{collections::VecDeque, iter};

use crate::{
    cells::Cells,
    ffi_out::random,
    input::{Input, InputEvent},
};

pub struct World {
    pub input: Input,
    pub positions: VecDeque<[i32; 2]>,
    pub velocity: [i32; 2],
    pub food_pos: [i32; 2],
    pub growth_left: i32,
    pub time_since_last_update_ms: f64,
    pub lost: bool,
    pub cells: Cells,
}

impl World {
    pub fn new(cells: Cells) -> Self {
        let x = cells.size[0] as i32 / 2;
        let y = cells.size[1] as i32 / 2;

        let mut self_ = Self {
            input: Input {
                events: VecDeque::new(),
            },
            positions: iter::once([x, y]).collect(),
            velocity: [1, 0],
            food_pos: [0, 0],
            growth_left: 2,
            time_since_last_update_ms: 0.,
            lost: false,
            cells,
        };

        self_.randomize_food_pos();

        self_
    }

    pub fn update(&mut self, delta_time_ms: f64) {
        if self.lost {
            return;
        }

        let delay_ms = 100.;

        self.time_since_last_update_ms += delta_time_ms;
        if self.time_since_last_update_ms >= delay_ms {
            self.time_since_last_update_ms = 0.;

            handle_input(self);
            move_snake(self);
            constrain_positions(self);
            check_collision(self);
            eat_food(self);
            update_cells(self);
        }
    }

    pub fn randomize_food_pos(&mut self) {
        self.food_pos = self
            .cells
            .size
            .map(|dim| (random() * dim as f32).floor() as i32);
    }

    pub fn head_position(&self) -> [i32; 2] {
        self.positions[0]
    }

    pub fn body_positions(&self) -> impl Iterator<Item = [i32; 2]> + '_ {
        self.positions.iter().copied().skip(1)
    }
}

fn handle_input(world: &mut World) {
    // Only process one input event per frame. That means, if the player presses
    // two keys in quick succession, they both have an effect, regardless of the
    // precise timing.
    //
    // It also prevents the case where two quick key presses could cause the
    // snake to reverse course, despite the explicit conditions here to prevent
    // just that.
    if let Some(input) = world.input.events.pop_front() {
        if input == InputEvent::Up && world.velocity != [0, 1] {
            world.velocity = [0, -1];
        }
        if input == InputEvent::Left && world.velocity != [1, 0] {
            world.velocity = [-1, 0];
        }
        if input == InputEvent::Down && world.velocity != [0, -1] {
            world.velocity = [0, 1];
        }
        if input == InputEvent::Right && world.velocity != [-1, 0] {
            world.velocity = [1, 0];
        }
    }
}

fn move_snake(state: &mut World) {
    let [mut head_x, mut head_y] = state.head_position();

    head_x += state.velocity[0];
    head_y += state.velocity[1];

    state.positions.push_front([head_x, head_y]);

    if state.growth_left > 0 {
        state.growth_left -= 1;
    } else {
        state.positions.pop_back();
    }
}

fn constrain_positions(state: &mut World) {
    for [x, y] in &mut state.positions {
        if *x < 0 {
            *x = state.cells.size[0] as i32 - 1;
        }
        if *x >= state.cells.size[0] as i32 {
            *x = 0;
        }
        if *y < 0 {
            *y = state.cells.size[1] as i32 - 1;
        }
        if *y >= state.cells.size[1] as i32 {
            *y = 0;
        }
    }
}

fn check_collision(state: &mut World) {
    let [head_x, head_y] = state.head_position();

    let mut lost = false;
    for [body_x, body_y] in state.body_positions() {
        if head_x == body_x && head_y == body_y {
            lost = true;
        }
    }

    state.lost = lost;
}

fn eat_food(state: &mut World) {
    let mut ate_food = false;

    for &[pos_x, pos_y] in &state.positions {
        if pos_x == state.food_pos[0] && pos_y == state.food_pos[1] {
            let extra_growth = state.positions.len() / 2;
            state.growth_left += extra_growth as i32;

            ate_food = true;
        }
    }

    if ate_food {
        state.randomize_food_pos();
    }
}

fn update_cells(state: &mut World) {
    for i in 0..state.cells.buffer.len() {
        state.cells.buffer[i] = 0;
    }

    for x in 0..state.cells.size[0] {
        for y in 0..state.cells.size[1] {
            let index = x + y * state.cells.size[0];

            if x as i32 == state.food_pos[0] && y as i32 == state.food_pos[1] {
                state.cells.buffer[index] = 127;
            }

            for &[pos_x, pos_y] in &state.positions {
                if x as i32 == pos_x && y as i32 == pos_y {
                    state.cells.buffer[index] = 255;
                }
            }
        }
    }
}
