//! # eoh-parser
//!
//! Recursive-descent parser for Eye of Horus.
//!
//! Consumes a flat [`Token`] stream produced by `eoh-lexer` and emits an
//! [`eoh_ast::Module`].  The parser is fully re-entrant and panic-free; all
//! errors are returned as [`EohError::Parse`].

#![deny(missing_docs)]
#![deny(unsafe_code)]

mod parser;

pub use parser::Parser;

use eoh_ast::Module;
use eoh_core::error::EohResult;
use eoh_lexer::Token;

/// Parse a token stream into an AST module.
///
/// `file_id` must match the id used during lexing.
pub fn parse(tokens: Vec<Token>, file_id: u32) -> EohResult<Module> {
    Parser::new(tokens, file_id).parse_module()
}
