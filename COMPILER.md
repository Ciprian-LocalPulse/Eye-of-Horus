# Compiler

## Status

Planned. Eye of Horus does not yet have a compiler in the sense of a pipeline that lowers source text to an independently persistable artifact ahead of execution; the current and immediately planned execution strategy is closer to a direct interpreter.

## The Likely First Implementation

The most likely first implementation is an interpreter composed of the pipeline stages already described in [`ARCHITECTURE.md`](ARCHITECTURE.md): a parser producing an AST, a static-checking pass, a spatial-matrix (field) builder, and a pulse runtime that executes directly against that field. This path is preferred for a research-stage language because it minimizes the number of independently specified artifacts (no separate bytecode format to keep synchronized with the AST) while the language core itself is still changing frequently.

## When a Compiler or Bytecode VM Would Be Justified

A compilation stage — lowering the AST to the bytecode instruction set already specified in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §6, whether executed by a dedicated VM crate or not (see [`VM.md`](VM.md)) — should be introduced only once the language core is stable enough to justify the additional layer. Concretely, this means:

- the declaration and expression grammar in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §3 is no longer expected to change frequently;
- the reserved-but-not-yet-executable constructs (`IF`/`ELSE`, `LOOP`/`BREAK`/`CONTINUE`, multi-file `IMPORT`) have been given full execution semantics, so that a compiler is not built against a moving target;
- there is a concrete, motivating use case — portability to a non-Rust host, ahead-of-time optimization, or a stable interchange format for tooling — rather than compilation as an end in itself.

## Relationship to Other Documents

This document intentionally defers to [`VM.md`](VM.md) for the question of bytecode dispatch and to [`ARCHITECTURE.md`](ARCHITECTURE.md) for the overall pipeline shape. It exists separately because "should Eye of Horus have a compiler at all, and when" is a distinct question from "what would the bytecode look like if it did," and conflating the two risks committing to an architecture before the underlying justification exists — precisely the ordering [Design Principle 2](DESIGN_PRINCIPLES.md#2-specification-before-stabilization) warns against.
