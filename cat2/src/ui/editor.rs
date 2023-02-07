use super::{
    area::{self, Area},
    border,
};

pub fn draw(area: Area, function: &str) {
    let mut area = border::draw(area);
    area::draw(&mut area, function);
}
