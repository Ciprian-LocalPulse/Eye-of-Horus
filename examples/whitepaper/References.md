# References

This is a preliminary bibliography supporting the white paper's positioning of Eye of Horus relative to prior work in non-linear, spatial, and esoteric programming languages. It should be expanded through ongoing research review rather than treated as complete; contributions that add a missing citation, correct an attribution, or point to more rigorous prior work are welcome under the documentation contribution guidelines in [`CONTRIBUTING.md`](../CONTRIBUTING.md).

## Esoteric and Non-Linear Languages

- Chris Pressey, Befunge language materials — the clearest direct ancestor of Eye of Horus's departure from single-instruction-pointer, linear execution, cited in the white paper's positioning section for its two-dimensional instruction grid.
- David Morgan-Mar, Piet language materials — an influential example of treating image geometry (color regions) as program structure rather than as incidental presentation.
- Esolang Wiki — community-maintained documentation of the broader esoteric-programming-language tradition, useful as a survey starting point rather than as a primary source for any single claim.

## Implementation Foundations

- The Rust Project — language and tooling documentation (`rustc`, `cargo`, `rustfmt`, `clippy`), underlying the implementation choices discussed in [`DESIGN_PRINCIPLES.md`](../DESIGN_PRINCIPLES.md) Principle 5.

## Broader Related Fields

- Literature on dataflow programming — relevant to the project's field-triggered activation model as an alternative to a single linear instruction pointer.
- Literature on visual programming — relevant to the planned visualizer and the open research question of measurable comprehension benefits, tracked in [`Future_Research.md`](Future_Research.md).
- Literature on computational geometry — relevant to the spatial primitives (vertices, shapes, fields) that form the language's core vocabulary.

## Citation Practice

Where a specific design decision in this project is directly motivated by a particular piece of prior work — as opposed to belonging to a broad tradition — that motivation should be documented in the relevant RFC, following the example set by RFC 0001's treatment of the golden ratio's continued-fraction properties, rather than left only to this general bibliography.
