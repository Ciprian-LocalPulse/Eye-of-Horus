# LSP Design Notes

## Status

Planned. This document records the intended capability set for a future Eye of Horus Language Server, so that implementation work — once it begins, per [`ROADMAP.md`](../ROADMAP.md) Phase 2 — has a stated target rather than starting from an unscoped blank slate.

## Planned Capabilities

- **Syntax diagnostics** — reporting parse errors with precise source spans, reusing the diagnostic infrastructure planned for the `eoh-parser` crate (see [`ARCHITECTURE.md`](../ARCHITECTURE.md)).
- **Semantic diagnostics** — reporting the static-check failures already specified in [`spec/LANGUAGE_SPEC.md`](../spec/LANGUAGE_SPEC.md) §4, such as unresolved names and shape arity violations, at the point in the source where they occur.
- **Hover information for geometry primitives** — showing a vertex's resolved coordinate, a shape's vertex count and kind, or a pulse's current parameters when the cursor hovers over the corresponding identifier.
- **Document symbols** — exposing declared vertices, shapes, functions, and pulses as navigable symbols in editor outline views.
- **Formatting** — a canonical formatter for `.eoh` source, once the surface syntax in [`SYNTAX.md`](../SYNTAX.md) is stabilized through the RFC process; formatting rules should not be finalized ahead of the syntax they format.
- **Visualizer integration hooks** — an interface allowing the planned spatial/pulse visualizer (see [`VISION.md`](../VISION.md)) to be launched or updated directly from editor commands, rather than requiring a separate standalone tool invocation.

## Dependency on the Language Core

Every capability above depends on the parser, static checker, or runtime reaching a level of completeness described in [`ROADMAP.md`](../ROADMAP.md) Phase 1. This document intentionally does not propose LSP-specific workarounds (such as a separate, simplified parser) to start development earlier, because maintaining two independent understandings of the grammar would violate [Design Principle 4](../DESIGN_PRINCIPLES.md#4-geometry-as-semantics-not-decoration)'s spirit of a single, authoritative semantic source.
