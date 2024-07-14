use thiserror::Error;

#[derive(Error, Debug)]
pub enum WasmError {
    #[error("Key loading error: {0}")]
    KeyLoadingError(String),

    #[error("Key saving error: {0}")]
    KeySavingError(String),

    #[error("Key generation error: {0}")]
    KeyGenerationError(String),

    #[error("public zk params generation error: {0}")]
    PublicZkParamsGenerationError(String),

    #[error("window object getting error: {0}")]
    WindowError(String),

    #[error("document object getting error: {0}")]
    DocumentError(String),

    #[error("local storage getting error: {0}")]
    LocalStorageError(String),
}

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("Binary operation error: {0}")]
    BinaryOpError(String),

    #[error("Unary Operation error: {0}")]
    UnaryOpError(String),
}
