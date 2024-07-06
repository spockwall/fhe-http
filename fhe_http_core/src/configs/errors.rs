use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsmError {
    #[error("Invalid instruction line: {0}")]
    InvalidInstruction(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),
}

#[derive(Error, Debug)]
pub enum FheError {
    #[error("Invalid key")]
    InvalidKey,

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),
}
