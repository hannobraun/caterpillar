use super::{
    area::{self, Area},
    vector::Vector,
};

pub const BORDER_OVERHEAD: usize = 2;

pub fn draw(mut area: Area) -> Area {
    draw_top(&mut area);
    draw_sides(&mut area);
    draw_bottom(&mut area);

    area::slice(area, [Vector { x: 1, y: 1 }, Vector { x: 2, y: 2 }])
}

fn draw_top(area: &mut area::Area) {
    draw_horizontal(area, "┏", "┓");
}

fn draw_sides(area: &mut Area) {
    let Vector { y: height, .. } = area::size(area);

    for _ in 1..height - 1 {
        draw_vertical(area);
        area::move_to_end_of_line(area);
        draw_vertical(area);
        area::move_to_next_line(area);
    }
}

fn draw_bottom(area: &mut area::Area) {
    draw_horizontal(area, "┗", "┛");
}

fn draw_horizontal(area: &mut Area, left_corner: &str, right_corner: &str) {
    let Vector { x: width, .. } = area::size(area);

    area::draw(area, left_corner);
    (0..width - BORDER_OVERHEAD).for_each(|_| area::draw(area, "━"));
    area::draw(area, right_corner);

    area::move_to_next_line(area);
}

fn draw_vertical(area: &mut Area) {
    area::draw(area, "┃");
}
