# Future Chapters

The following chapters are planned, in the order introduced by [`book/00-introduction.md`](../00-introduction.md)'s "What's next" section:

- **Chapter 1 — Vertices and Shapes.** Declaring vertices, edges, and the built-in shape kinds (tetrahedron, cube, icosahedron, sphere), including the arity constraints specified in [`spec/LANGUAGE_SPEC.md`](../../spec/LANGUAGE_SPEC.md) §4.2.
- **Chapter 2 — The Spatial Field.** How the phi-pi addressing lattice works in practice, and what it means for two vertices to share storage; a practical companion to [`MEMORY_MODEL.md`](../../MEMORY_MODEL.md).
- **Chapter 3 — Pulses and Activation.** The Higgs-pulse model in depth, with worked timing diagrams illustrating the semantics formalized in [`spec/SEMANTICS.md`](../../spec/SEMANTICS.md).
- **Chapter 4 — Functions and Arithmetic.** The `FN`, `LET`, and expression grammar, and how they interact with the spatial field.

Each chapter will be written and merged only once its subject matter is stable enough in the reference implementation to keep the chapter's examples from becoming outdated, per [Design Principle 3](../../DESIGN_PRINCIPLES.md#3-tests-before-claims). Proposals for additional chapters — for example, a chapter specifically on debugging with the planned visualizer — are welcome as GitHub Discussions; see [`SUPPORT.md`](../../SUPPORT.md).
