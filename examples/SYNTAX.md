# Syntax

## Status

Draft, partially stabilized through the formal grammar in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §3. This document is a readable walkthrough of the surface syntax for readers who want an illustrated tour before consulting the EBNF grammar directly; where the two disagree, the formal grammar governs.

## Illustrative Example

```eoh
ORIGIN 0.0, 0.0, 0.0
VERTEX A 1.0, 1.618, 3.141
VECTOR FLOW A -> B
SHAPE_TETRA T1 A, B, C, D
PULSE_HIGGS ORIGIN, v=1.0
```

This example is illustrative of the target surface syntax, not a claim that every line above executes against the current runtime; `VECTOR FLOW` in particular is written here in the informal style used throughout early project documentation, while the normative grammar in `spec/LANGUAGE_SPEC.md` currently specifies directed relationships through `EDGE ident -> ident`. Where informal examples in this repository use a notation not yet present in the formal grammar, that notation should be read as a proposal for a future RFC rather than as documented behavior.

## Declaration Forms

The formally specified declaration forms are:

- `ORIGIN x, y, z` — declares the coordinate-space origin.
- `VERTEX name x, y, z` — binds `name` to a validated coordinate.
- `EDGE a -> b` — declares a directed relationship between two named vertices.
- `SHAPE_TETRA name v1, v2, v3, v4` — a four-vertex solid (exact arity required).
- `SHAPE_CUBE name anchor, size=<float>` — a cube anchored at one vertex.
- `SHAPE_ICOSA name v1 .. v12` — a twelve-vertex solid (exact arity required).
- `SHAPE_SPHERE name center, r=<float>` — a sphere given a center and radius.
- `PULSE_HIGGS origin, v=<float>` — emits an isotropically expanding activation pulse from `origin` at the given velocity.
- `LET name [: Type] = expr` — binds a computed value.
- `FN name(param: Type, ...) [-> Type] { ... }` — declares a function.
- `IMPORT "path"` — reserved for future multi-file module resolution.

The full expression grammar (comparison, additive, multiplicative, and unary operators, function calls, and parenthesized sub-expressions) is given precisely in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §3 and is not repeated here to avoid the two documents drifting out of sync.

## Existing Implementation

No complete parser exists yet covering every construct above end to end with full diagnostic reporting. The declaration and expression grammar has been specified ahead of full parser implementation, per [Design Principle 2](DESIGN_PRINCIPLES.md#2-specification-before-stabilization); readers should consult [`ARCHITECTURE.md`](ARCHITECTURE.md) for the current state of the `eoh-parser` scaffold and [`ROADMAP.md`](ROADMAP.md) Phase 1 for its tracked completion.

## Planned Implementation

Syntax is stabilized through the RFC process described in [`RFC_PROCESS.md`](RFC_PROCESS.md), backed by an expanding parser test corpus. A change to any of the declaration forms above, or an addition of a new one, requires an RFC.

## Future RFCs

The following syntax areas are known to require dedicated RFCs before they can be considered stable:

- comments and whitespace handling beyond the current line-comment (`//`) rule;
- identifier and namespace rules, including whether Unicode identifiers will be supported (see [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §2.1);
- the numeric literal grammar, including scientific notation edge cases;
- the full `SHAPE_POLY` declaration syntax for arbitrary polygons;
- module and import syntax for multi-file programs.
