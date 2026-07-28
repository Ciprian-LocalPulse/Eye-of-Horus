# Eye of Horus — Grammar Quick Reference

**Companion to:** [`LANGUAGE_SPEC.md`](LANGUAGE_SPEC.md)

## Purpose and Scope of This Page

This page is a deliberately short, non-normative quick reference to the surface grammar — useful as a one-page reminder while reading or writing `.eoh` source — and is not an independent specification. Every claim here is derived from, and subordinate to, [`LANGUAGE_SPEC.md`](LANGUAGE_SPEC.md); if the two ever appear to disagree, that document governs and this page has a bug that should be reported.

## 1. Lexical Structure

UTF-8 source; ASCII-only identifiers for now (`[A-Za-z_][A-Za-z0-9_]*`); `//` line comments; no block comments. Full detail: [`LANGUAGE_SPEC.md`](LANGUAGE_SPEC.md) §2.

## 2. Grammar

Declarations: `ORIGIN`, `VERTEX`, `EDGE`, `SHAPE_TETRA` / `SHAPE_CUBE` / `SHAPE_ICOSA` / `SHAPE_SPHERE`, `PULSE_HIGGS`, `LET`, `FN`, `IMPORT`. Full EBNF: [`LANGUAGE_SPEC.md`](LANGUAGE_SPEC.md) §3, illustrated informally in [`SYNTAX.md`](../SYNTAX.md).

## 3. Names and Binding

Every declaration introduces a module-scope binding; v0.1 has no nested lexical scoping beyond function bodies. Full detail: [`LANGUAGE_SPEC.md`](LANGUAGE_SPEC.md) §4.1.

## 4. Values

`Float`, `Bool`, `Str`, `Vertex`, `Shape` — a minimal, structural type subset, not yet a full inference system. Full detail: [`LANGUAGE_SPEC.md`](LANGUAGE_SPEC.md) §4.3 and open questions in [`TYPE_SYSTEM.md`](../TYPE_SYSTEM.md).

## 5. Geometry Primitives

Vertex-count constraints per shape kind (`SHAPE_TETRA`: 4, `SHAPE_ICOSA`: 12, `SHAPE_CUBE`/`SHAPE_SPHERE`: 1 anchor plus a parameter). Full detail: [`LANGUAGE_SPEC.md`](LANGUAGE_SPEC.md) §4.2.

## 6. Execution

Pulse-activation model: a point is activated once a live pulse's expanding radius reaches it. Full informal treatment: [`SEMANTICS.md`](../SEMANTICS.md). Full formal treatment: [`SEMANTICS.md`](SEMANTICS.md) (this directory).

## 7. Memory Model

Values are stored by phi-pi lattice address, not by raw coordinate. Full detail: [`MEMORY_MODEL.md`](../MEMORY_MODEL.md) and RFC 0001.

## 8. Errors

Geometry errors (arity violations) and resolution errors are reported before execution; `Div` faults at runtime on a zero divisor. Full detail: [`LANGUAGE_SPEC.md`](LANGUAGE_SPEC.md) §4 and §6, and [`SEMANTICS.md`](SEMANTICS.md) (this directory) §4.

## 9. Conformance

There is no independent conformance suite yet; the reference implementation's test corpus is the current de facto conformance target. A dedicated conformance test suite, independent of the reference implementation's internal test structure, is future work tracked in [`ROADMAP.md`](../ROADMAP.md).
