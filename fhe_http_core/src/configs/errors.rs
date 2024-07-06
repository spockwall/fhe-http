use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsmError {
    #[error("Invalid instruction line: {0}")]
    InvalidInstruction(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),
}
