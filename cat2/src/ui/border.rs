use super::{
    area::{self, Area},
    vector::Vector,
};

pub fn write(mut area: Area) -> anyhow::Result<Area> {
    write_top(&mut area)?;

    let Vector { y: height, .. } = area::size(&area);
    for _ in 1..height - 1 {
        print_vertical(&mut area)?;
        area::move_to_end_of_line(&mut area);
        print_vertical(&mut area)?;
        area::move_to_new_line(&mut area);
    }

    print_bottom(&mut area)?;

    let offset = Vector { x: 1, y: 1 };
    Ok(area::slice(area, [offset, offset]))
}

fn write_top(area: &mut area::Area) -> anyhow::Result<()> {
    print_horizontal(area, "┏", "┓")
}

fn print_bottom(area: &mut area::Area) -> anyhow::Result<()> {
    print_horizontal(area, "┗", "┛")
}

fn print_horizontal(
    area: &mut Area,
    left_corner: &str,
    right_corner: &str,
) -> anyhow::Result<()> {
    let Vector { x: width, .. } = area::size(area);

    area::write(area, left_corner)?;
    (0..width).try_for_each(|_| area::write(area, "━"))?;
    area::write(area, right_corner)?;

    area::move_to_new_line(area);

    Ok(())
}

fn print_vertical(area: &mut Area) -> anyhow::Result<()> {
    area::write(area, "┃")
}
