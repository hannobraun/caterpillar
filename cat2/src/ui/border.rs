use super::area;

pub fn print_vertical_border(area: &mut area::Area) -> anyhow::Result<()> {
    area::write(area, "┃")
}
