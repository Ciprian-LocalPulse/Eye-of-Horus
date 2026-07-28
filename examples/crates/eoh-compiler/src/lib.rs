//! # eoh-compiler
//!
//! The Eye of Horus compiler pipeline.
//!
//! Stages:
//! 1. **Lex** — tokenise source text (`eoh-lexer`)
//! 2. **Parse** — produce a typed AST (`eoh-parser`)
//! 3. **Resolve** — bind names, resolve imports (`resolver`)
//! 4. **Type-check** — enforce the type system (`typeck`)
//! 5. **Lower** — convert AST to Mid-level IR (`lower`)
//! 6. **Optimise** — run MIR passes (`optimise`)
//! 7. **Emit** — serialise to EOH bytecode (`emit`)

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod bytecode;
pub mod emit;
pub mod lower;
pub mod optimise;
pub mod resolver;
pub mod typeck;

use eoh_ast::Module;
use eoh_core::error::{EohError, EohResult};
use eoh_lexer::lex;
use eoh_parser::parse;

/// Compilation options.
#[derive(Debug, Clone, Default)]
pub struct CompileOptions {
    /// Emit verbose diagnostic output.
    pub verbose: bool,
    /// Optimisation level (0 = none, 1 = basic, 2 = full).
    pub opt_level: u8,
    /// Write human-readable MIR to this path (if `Some`).
    pub dump_mir: Option<String>,
}

/// The result of a successful compilation.
#[derive(Debug)]
pub struct CompileOutput {
    /// The parsed and type-checked module (retained for IDE tooling).
    pub module: Module,
    /// The final bytecode image.
    pub bytecode: bytecode::BytecodeImage,
}

/// Compile Eye of Horus source text to a bytecode image.
///
/// This is the primary public entry point.
pub fn compile(source: &str, file_id: u32, opts: &CompileOptions) -> EohResult<CompileOutput> {
    // Stage 1 — Lex
    let tokens = lex(source, file_id)?;
    if opts.verbose {
        log::debug!("lexed {} tokens", tokens.len());
    }

    // Stage 2 — Parse
    let module = parse(tokens, file_id)?;
    if opts.verbose {
        log::debug!("parsed {} top-level items", module.items.len());
    }

    // Stage 3 — Resolve
    let resolved = resolver::resolve(&module)?;

    // Stage 4 — Type-check
    typeck::check(&resolved)?;

    // Stage 5 — Lower to MIR
    let mut mir = lower::lower(&resolved)?;
    if let Some(path) = &opts.dump_mir {
        let json = serde_json::to_string_pretty(&mir)
            .map_err(|e| EohError::Io(e.to_string()))?;
        std::fs::write(path, json).map_err(|e| EohError::Io(e.to_string()))?;
    }

    // Stage 6 — Optimise
    if opts.opt_level > 0 {
        optimise::run(&mut mir, opts.opt_level);
    }

    // Stage 7 — Emit bytecode
    let bytecode = emit::emit(&mir)?;

    Ok(CompileOutput { module, bytecode })
}
