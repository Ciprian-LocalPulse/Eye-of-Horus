# Specification

This directory contains the formal specification for Eye of Horus. It is the normative source of truth for the language's grammar and semantics; where any other document in this repository — including the bridging summaries at the repository root — appears to disagree with a document here, the document in this directory governs, and the discrepancy should be reported as an issue.

## Contents

- [`LANGUAGE_SPEC.md`](LANGUAGE_SPEC.md) — the language specification proper: scope, lexical structure, the formal EBNF grammar, static semantics (name resolution, shape-arity constraints, the current type subset, coordinate domain constraints), the bytecode instruction set, and bytecode schema versioning.
- [`SEMANTICS.md`](SEMANTICS.md) — a small-step operational semantics for the executable core, written in SOS-style judgments, including an explicit, itemized section of known semantic gaps in the current reference implementation.
- [`language.md`](language.md) — a short, non-normative quick reference to the surface grammar, useful as a one-page reminder; see that file's own note on its relationship to `LANGUAGE_SPEC.md`.

## Editorial Policy

Every section in this directory that is not yet complete is marked explicitly rather than left silently blank, per [Design Principle 1](../DESIGN_PRINCIPLES.md#1-honesty-before-spectacle). As of this writing, the core lexical structure, grammar, static semantics, and operational semantics for the currently executable language subset are substantially specified; reserved-but-not-yet-executable constructs (`IF`/`ELSE`, `LOOP`/`BREAK`/`CONTINUE`, multi-file `IMPORT`, full `SHAPE_POLY`) are named and tracked but not yet given complete semantics, per [`LANGUAGE_SPEC.md`](LANGUAGE_SPEC.md) §8.

## Changing This Specification

Any change to a document in this directory is, by definition, a specification change and requires an RFC under the categories listed in [`RFC_PROCESS.md`](../RFC_PROCESS.md). See [`LANGUAGE_SPECIFICATION.md`](../LANGUAGE_SPECIFICATION.md) at the repository root for a reader-oriented overview of this directory's contents before diving into the normative text itself.
