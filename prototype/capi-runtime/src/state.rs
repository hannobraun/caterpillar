use crate::{evaluator::Evaluator, render_target::RenderTarget, world::World};

pub struct State {
    pub evaluator: Evaluator,
    pub world: World,
    pub render_target: RenderTarget,
}
