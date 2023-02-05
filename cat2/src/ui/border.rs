use super::{
    area::{self, Area},
    vector::Vector,
};

pub fn write(mut area: Area) -> anyhow::Result<Area> {
    write_top(&mut area)?;
    write_sides(&mut area)?;
    draw_bottom(&mut area)?;

    let offset = Vector { x: 1, y: 1 };
    Ok(area::slice(area, [offset, offset]))
}

fn write_top(area: &mut area::Area) -> anyhow::Result<()> {
    draw_horizontal(area, "┏", "┓");
    Ok(())
}

fn write_sides(area: &mut Area) -> anyhow::Result<()> {
    let Vector { y: height, .. } = area::size(area);

    for _ in 1..height - 1 {
        draw_vertical(area);
        area::move_to_end_of_line(area);
        draw_vertical(area);
        area::move_to_new_line(area);
    }

    Ok(())
}

fn draw_bottom(area: &mut area::Area) -> anyhow::Result<()> {
    draw_horizontal(area, "┗", "┛");
    Ok(())
}

fn draw_horizontal(area: &mut Area, left_corner: &str, right_corner: &str) {
    let Vector { x: width, .. } = area::size(area);

    area::draw(area, left_corner);
    (0..width - 2).for_each(|_| area::draw(area, "━"));
    area::draw(area, right_corner);

    area::move_to_new_line(area);
}

fn draw_vertical(area: &mut Area) {
    area::draw(area, "┃");
}
