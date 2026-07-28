# Language Specification

## Status and Scope of This Document

This is the entry point to Eye of Horus's language specification, not a duplicate of it. It exists to orient a reader before they descend into the normative documents in [`spec/`](spec/README.md), and to state plainly which parts of the language those documents already cover in full and which parts remain unspecified.

The two normative documents are:

- [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) — lexical structure, formal grammar (EBNF), static semantics (name resolution, shape-arity constraints, the current type subset, coordinate domain constraints), the bytecode instruction set, and bytecode schema versioning.
- [`spec/SEMANTICS.md`](spec/SEMANTICS.md) — a small-step operational semantics for the executable core, written in SOS-style judgments, including an explicit section of known semantic gaps.

Where this document and those two disagree, the documents in `spec/` are authoritative; please file an issue if a discrepancy is found, per [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §1.

## What Is Specified Today

As of this writing, the normative specification fully covers:

- the lexical grammar, including the deliberate decision not to give negative numeric literals their own lexical class (see [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §2.3);
- the EBNF grammar for declarations, statements, and expressions, including `ORIGIN`, `VERTEX`, `EDGE`, the `SHAPE_*` family, `PULSE_HIGGS`, `LET`, `FN`, and `IMPORT`;
- name resolution rules and shape vertex-count constraints;
- the current, intentionally minimal type subset (`Float`, `Bool`, `Str`, `Vertex`, `Shape`) and an explicit statement that full cross-function type inference is not yet implemented;
- coordinate domain constraints (`Coord3D::new` enforces finiteness and `|component| <= MAX_COORD`);
- the pulse-activation operational model, both informally (§5 of the language spec) and formally (the full semantics document);
- a stack-based bytecode instruction set and its versioning policy.

## What Remains Unspecified or Reserved

The keyword table and grammar reserve several constructs that parse today but do not yet have complete execution semantics: `IF` / `ELSE` conditional execution, `LOOP` / `BREAK` / `CONTINUE` iteration, multi-file `IMPORT` resolution, and full `SHAPE_POLY` geometry beyond vertex-count validation. See [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §8 for the complete list and its tracking in [`ROADMAP.md`](ROADMAP.md). Treating a reserved-but-not-yet-load-bearing construct as though it already executed is a documentation error; see [Design Principle 1](DESIGN_PRINCIPLES.md#1-honesty-before-spectacle).

## Companion Document

[`spec/language.md`](spec/language.md) is maintained separately as a short, non-normative quick-reference to the grammar, intended for readers who want a one-page reminder of the surface syntax rather than the full specification with its accompanying rationale. It is not an independent specification and should never be read as authoritative where it appears to conflict with `spec/LANGUAGE_SPEC.md`.

## Changing the Specification

Any change to lexical structure, grammar, static semantics, or the bytecode instruction set is a specification change and requires an RFC under the categories listed in [`RFC_PROCESS.md`](RFC_PROCESS.md). Editorial clarifications that do not change meaning may be made directly, consistent with the governance rules in [`GOVERNANCE.md`](GOVERNANCE.md).
