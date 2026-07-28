//! # eoh-lsp
//!
//! Language Server Protocol implementation for Eye of Horus.
//!
//! Provides IDE features (diagnostics, hover, completion, go-to-definition)
//! for `.eoh` files via the LSP 3.17 protocol over stdin/stdout JSON-RPC.
//!
//! **Status:** placeholder structure — wire-up and full handler implementation
//! is tracked in Phase 3 of the roadmap.

#![deny(missing_docs)]

pub mod capabilities;
pub mod diagnostics;
pub mod handlers;
pub mod server;

pub use server::LspServer;
