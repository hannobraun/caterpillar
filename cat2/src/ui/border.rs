use super::area::{self, Area};

pub fn print_top(area: &mut area::Area) -> anyhow::Result<()> {
    print_horizontal(area, "┏", "┓")
}

pub fn print_bottom(area: &mut area::Area) -> anyhow::Result<()> {
    print_horizontal(area, "┗", "┛")
}

fn print_horizontal(
    area: &mut Area,
    left_corner: &str,
    right_corner: &str,
) -> anyhow::Result<()> {
    let [width, _] = area::size(area);

    area::write(area, left_corner)?;
    (0..width).try_for_each(|_| area::write(area, "━"))?;
    area::write(area, right_corner)?;

    area::new_line(area);

    Ok(())
}

pub fn print_vertical(area: &mut Area) -> anyhow::Result<()> {
    area::write(area, "┃")
}
