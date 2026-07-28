# Type System

## Status

Partially implemented; substantially open as a research and design question. This document distinguishes the small, structural type subset already present in the reference implementation from the larger open questions that a future RFC must resolve before the type system can be called stable.

## What Exists Today

Per [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §4.3, the current implementation recognizes a minimal, mostly structural set of kinds:

- `Float` — an IEEE-754 double, validated finite and, when used as a spatial coordinate component, bounded by `MAX_COORD`;
- `Bool`;
- `Str`;
- `Vertex` — the implicit type of a `VERTEX` declaration, carrying a validated coordinate;
- `Shape` — the implicit type of a `SHAPE_*` declaration.

Today's checker performs **structural validation**, not full type inference: shape vertex-count checks and `PULSE_HIGGS` origin-existence checks are enforced, but there is no Hindley–Milner-style inference across function boundaries. This limitation is stated here exactly as it is stated in the specification and the white paper's Open Problems section, to avoid any drift between documents on this point.

## Open Questions

The following are genuinely unresolved and are listed here as questions rather than as design decisions already made, in keeping with [Design Principle 2](DESIGN_PRINCIPLES.md#2-specification-before-stabilization):

- **Are coordinates themselves values, references, or both?** A `VERTEX` binds a name to a coordinate, but it is not yet settled whether reading that name should be understood as reading a first-class value, dereferencing a location, or some hybrid the language must define explicitly to avoid ambiguity in future constructs such as vertex aliasing.
- **Are geometric primitives typed by dimension?** Whether a `Shape` built from three-dimensional vertices should be distinguished, at the type level, from a hypothetical lower-dimensional analog is open, and has consequences for how strictly the arity checks in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §4.2 can be generalized.
- **Does the language need scalar, vector, matrix, field, and shape kinds as a unified hierarchy**, rather than the current flat, ad hoc set of five kinds? A unified hierarchy would likely simplify future standard-library design (see [`STANDARD_LIBRARY.md`](STANDARD_LIBRARY.md)) at the cost of a larger up-front specification effort.
- **Should type inference exist in early versions at all**, or should the language remain fully explicitly typed until the standard library and function-composition patterns are better understood empirically? Introducing inference prematurely risks specifying rules that later prove awkward once real programs exist to test them against.

## Constraints on Future Work

Any RFC proposing a full type system must, at minimum, address:

- how the existing structural checks (vertex arity, coordinate validity) compose with or are subsumed by the new system, so that no currently-enforced invariant is silently weakened;
- whether type errors are reported before or interleaved with the static checks already specified in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §4;
- how the type system interacts with the bytecode instruction set in §6 of that same document, since several instructions (`Add`, `Sub`, `Div`) currently operate on untyped stack values with runtime rather than static failure modes (see the `Div` fault rule in [`spec/SEMANTICS.md`](spec/SEMANTICS.md) §4).

## Relationship to Other Documents

This document intentionally says less than [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §4.3 about what is implemented, and more than that document about what remains open, because its purpose is to serve as a landing page for anyone considering writing a type-system RFC. Read the specification first for precise current behavior; read this document second for the shape of the problem still to be solved.
