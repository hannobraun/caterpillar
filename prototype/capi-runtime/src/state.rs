use crate::{
    cells::Cells, render_target::RenderTarget, vm::Evaluator, world::World,
};

pub struct State {
    pub evaluator: Evaluator,
    pub world: World,
    pub render_target: RenderTarget,
}

impl State {
    pub fn new(width: usize, height: usize, data: &[u8]) -> Self {
        let render_target = RenderTarget::new(width, height);
        let cells = Cells::new(&render_target);
        let state = World::new(cells);

        Self {
            evaluator: Evaluator::new(data),
            world: state,
            render_target,
        }
    }
}
