use crate::{draw_target::RenderTarget, world::World};

pub struct State {
    pub world: World,
    pub render_target: RenderTarget,
}
