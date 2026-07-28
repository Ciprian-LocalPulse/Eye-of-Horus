# Future Research

This page lists open research directions raised by Eye of Horus's design but not resolved by the current specification or white paper draft. Each item here is cross-referenced to the document where it is tracked in more implementation-relevant terms, so that this page serves as a research-oriented index rather than a duplicate task list.

- **Formal semantics.** Extending the small-step operational semantics in [`spec/SEMANTICS.md`](../spec/SEMANTICS.md) to cover `IF`/`ELSE`, `LOOP`, and multi-file `IMPORT` once those constructs are given execution behavior, and formally verifying properties such as determinism of activation ordering.
- **Computational power analysis.** Resolving whether Eye of Horus, or a specified subset of it, is Turing-complete — an explicitly open question in the white paper's Open Problems section, requiring either a constructive reduction or a proof of limitation.
- **Deterministic scheduling.** A principled tie-breaking rule for simultaneous pulse activation, addressing the "same-radius activation as deterministic batching" research idea in [`ARCHITECTURE.md`](../ARCHITECTURE.md).
- **Visual programming interfaces.** Design and evaluation of the planned visualizer described in [`VISION.md`](../VISION.md), including what visual encoding best represents an activation field without misleading a viewer about its underlying discrete computation.
- **Geometry-aware debugging.** Debugger designs that let a programmer step through pulse propagation and inspect the spatial field's lattice occupancy directly, building on the tracing and debugging requirements listed in [`RUNTIME.md`](../RUNTIME.md).
- **Teaching materials for programming-language courses.** Course-style modules that use Eye of Horus as a vehicle for teaching alternative computation models, complementing the tutorial-oriented [language book](../book/README.md).
- **Controlled studies of spatial code comprehension.** Empirical, human-subjects research measuring whether spatial/visual representations of program state improve comprehension relative to conventional textual debugging, as opposed to merely asserting that they do — the central empirical question behind the project's visual-debugging ambitions, tracked as [`ROADMAP.md`](../ROADMAP.md) Phase 4 work.

## A Note on Scope

Several of the items above require resources — human-subjects research review, dedicated benchmarking infrastructure — beyond what a small research project can commit to on a fixed timeline. They are recorded here as legitimate open questions the project would welcome outside collaboration on, not as commitments with an implied schedule.
