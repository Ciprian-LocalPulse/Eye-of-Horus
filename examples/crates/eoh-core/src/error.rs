//! Canonical error type for all Eye of Horus crates.

use thiserror::Error;

/// Every error that can arise in the Eye of Horus toolchain.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum EohError {
    /// A coordinate or geometry value was outside the valid domain.
    #[error("geometry error: {0}")]
    Geometry(String),

    /// The lexer could not tokenise the input.
    #[error("lexer error at {location}: {message}")]
    Lex {
        /// Human-readable source location.
        location: String,
        /// Description of the failure.
        message: String,
    },

    /// The parser could not produce a valid AST.
    #[error("parse error at {location}: {message}")]
    Parse {
        /// Human-readable source location.
        location: String,
        /// Description of the failure.
        message: String,
    },

    /// A type-check or semantic constraint was violated.
    #[error("type error: {0}")]
    Type(String),

    /// The virtual machine encountered a runtime fault.
    #[error("runtime fault: {0}")]
    Runtime(String),

    /// An I/O problem (file not found, permissions, etc.).
    #[error("I/O error: {0}")]
    Io(String),

    /// A feature that is planned but not yet implemented.
    #[error("not yet implemented: {0}")]
    NotImplemented(String),
}

/// Convenience alias used throughout the toolchain.
pub type EohResult<T> = Result<T, EohError>;
