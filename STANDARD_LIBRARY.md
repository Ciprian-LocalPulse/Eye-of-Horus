# Standard Library

## Status

Planned. No standard library module has yet been specified or implemented to the standard required by [Design Principle 3](DESIGN_PRINCIPLES.md#3-tests-before-claims): backed by tests and documentation before being described as available.

## Design Constraint

Any standard-library module accepted into this project must, at minimum:

- have a corresponding specification entry describing its public interface and semantics, consistent with [Design Principle 2](DESIGN_PRINCIPLES.md#2-specification-before-stabilization);
- be backed by automated tests exercising every documented behavior;
- go through the RFC process described in [`RFC_PROCESS.md`](RFC_PROCESS.md), since standard-library design is explicitly listed there as requiring RFC discussion.

A module that exists as source code without both of the above is, for documentation purposes, not yet part of the standard library and should not be described as such in user-facing material.

## Possible Future Areas

The following areas are plausible candidates for a future standard library, listed here as design space to be explored rather than as commitments:

- **Numeric utilities** — general-purpose floating-point helpers beyond the coordinate-validation logic already present in `eoh-core` (see [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §4.4).
- **Geometry primitives** — reusable constructions such as the Fibonacci-sphere point distribution and golden-angle constant already referenced informally in RFC 0001, if and when they are promoted from internal scaffolding to a documented, tested public interface.
- **Field operations** — higher-level operations over the spatial field (querying occupancy, iterating active regions) that would otherwise need to be reimplemented by every program that wants them.
- **Visualization hooks** — a stable interface for the runtime's event stream to feed the visualizer described in [`VISION.md`](VISION.md), without requiring the visualizer to depend on runtime internals.
- **Educational examples** — worked, tested `.eoh` programs illustrating each standard-library module, complementing the tutorial material in the [language book](book/00-introduction.md).

## Relationship to the Type System

Because the type system itself is still an open design question (see [`TYPE_SYSTEM.md`](TYPE_SYSTEM.md)), standard-library design necessarily depends on decisions not yet made — for instance, whether the library exposes a unified scalar/vector/matrix/field/shape hierarchy depends on whether such a hierarchy is adopted at the type-system level. Standard-library RFCs should state explicitly which open type-system questions they depend on, rather than silently assuming an answer.
