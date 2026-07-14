# Language Specification

Status: Version 0.1 draft skeleton.

This document is a bridge to the formal specification in [spec/language.md](spec/language.md). It describes current intent while marking unfinished work as TODO.

## Existing Implementation

The current Rust code does not implement the full language.

## Planned Implementation

Eye of Horus source files use the `.eoh` extension. The language is expected to define origins, vertices, vectors, shapes, fields, and pulse-triggered execution.

## TODO

- lexical grammar;
- parse tree and AST;
- value model;
- deterministic ordering for simultaneous activation;
- error handling;
- module system;
- test corpus.
