pub mod stages;
pub mod stage_input;

#[derive(Debug, thiserror::Error)]
pub enum PipelineError<T> {
    #[error(transparent)]
    NotEnoughInput(#[from] stage_input::NoMoreInput),

    #[error(transparent)]
    Stage(T),
}
