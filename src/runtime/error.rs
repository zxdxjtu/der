use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum RuntimeError {
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch {
        expected: String,
        actual: String,
    },

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Unknown opcode: {0}")]
    UnknownOpcode(u16),

    #[error("Stack underflow")]
    StackUnderflow,

    #[error("Invalid node reference: {0}")]
    InvalidNodeRef(u32),

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Invalid argument count: expected {expected}, got {actual}")]
    InvalidArgCount {
        expected: usize,
        actual: usize,
    },

    #[error("Missing capability: {0:?}")]
    MissingCapability(crate::core::Capability),

    #[error("Invalid constant index: {0}")]
    InvalidConstantIndex(u32),

    #[error("Array index out of bounds: {index} for array of length {length}")]
    ArrayIndexOutOfBounds {
        index: usize,
        length: usize,
    },

    #[error("Map key not found: {0}")]
    MapKeyNotFound(String),

    #[error("Maximum call depth exceeded")]
    StackOverflow,

    #[error("IO error: {0}")]
    IOError(String),

    #[error("External call failed: {0}")]
    ExternalCallFailed(String),

    #[error("Proof verification failed: {0}")]
    ProofVerificationFailed(String),
}

pub type Result<T> = std::result::Result<T, RuntimeError>;