# Acknowledgements

Eye of Horus acknowledges the broader programming-language, esoteric-language ("esolang"), compiler, computational-geometry, and Rust communities, whose public work makes a project like this possible to attempt responsibly.

## Intellectual Lineage

The project is especially informed by prior work in:

- **Spatial and non-linear programming languages** — most directly, Befunge's treatment of the source file as a two-dimensional instruction grid with a movable instruction pointer, cited explicitly in the white paper's positioning section as the clearest ancestor of Eye of Horus's own departure from purely linear execution.
- **Image- and geometry-based execution models** — Piet's use of color regions and image geometry as program structure, an early and influential example of treating a program's spatial layout as semantically meaningful rather than incidental.
- **Visual and dataflow programming** — the broader tradition of representing computation as a graph or diagram rather than as ordered text, which informs the project's long-term visualizer ambitions described in [`VISION.md`](VISION.md).
- **Educational language design** — the practice, common across teaching-oriented languages, of prioritizing comprehensibility and honest documentation of limitations over performance or industrial adoption, which this project explicitly adopts as a value in [`MANIFESTO.md`](MANIFESTO.md).
- **The Rust project and its tooling ecosystem** — `cargo`, `rustfmt`, and `clippy`, which make the small-crate, test-driven development style described in [`DESIGN_PRINCIPLES.md`](DESIGN_PRINCIPLES.md) practical to sustain.

## Community Documentation Practices

The project's governance and contribution documents draw on widely adopted open-source community norms, including the Contributor Covenant framework referenced in [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) and the Keep a Changelog convention used in [`CHANGELOG.md`](CHANGELOG.md).

## A Living Document

This list is intentionally incomplete and will grow as contributors join the project and as specific design decisions are traced back to particular prior work — for example, RFC 0001 credits the specific mathematical properties of the golden ratio that motivate the phi-pi addressing scheme, rather than repeating that attribution here in general terms.
