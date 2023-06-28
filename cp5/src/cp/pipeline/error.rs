use super::stage_input::NoMoreInput;

#[derive(Debug, thiserror::Error)]
pub enum PipelineError<T> {
    #[error(transparent)]
    NotEnoughInput(#[from] NoMoreInput),

    #[error(transparent)]
    Stage(T),
}
