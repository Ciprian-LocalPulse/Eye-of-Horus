//! # eoh-lexer
//!
//! Tokeniser for Eye of Horus (`.eoh`) source files.
//!
//! The lexer transforms a raw UTF-8 source string into a flat list of
//! [`Token`] values that the parser can consume in a single forward pass.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod token;
pub mod lexer;

pub use lexer::Lexer;
pub use token::{Token, TokenKind};

use eoh_core::error::EohResult;

/// Lex an entire source string into a token list.
///
/// `file_id` is the index of this file in the compiler's source-file table.
/// Pass `0` for anonymous input (REPL, tests).
pub fn lex(source: &str, file_id: u32) -> EohResult<Vec<Token>> {
    Lexer::new(source, file_id).collect_all()
}
