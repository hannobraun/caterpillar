use crate::{
    cells::Cells, evaluator::Evaluator, render_target::RenderTarget,
    world::World,
};

pub struct State {
    pub evaluator: Evaluator,
    pub world: World,
    pub render_target: RenderTarget,
}

impl State {
    pub fn new(width: usize, height: usize) -> Self {
        let render_target = RenderTarget::new(width, height);
        let cells = Cells::new(&render_target);
        let state = World::new(cells);

        Self {
            evaluator: Evaluator::new(&[
                b'c', b'p', 0, b'S', b'c', b'p', 1, b'S', b'p', 2, b'S', b'p',
                255, b'p', 3, b'S', b't',
            ]),
            world: state,
            render_target,
        }
    }
}
