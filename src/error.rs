use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuckshotError {
    #[error("Invalid argument.")]
    ValueError,
}