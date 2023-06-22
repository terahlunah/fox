use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Stack error: {0}")]
    StackError(String),
    #[error("Downcast error: expected {0}")]
    CastError(String),
    #[error("The symbol `{0}` is not defined")]
    UnknownSymbol(String),
}
