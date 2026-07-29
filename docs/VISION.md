# Vision

## Purpose of This Document

This document states, without exaggeration, what Eye of Horus is trying to become, why that goal is worth pursuing, and what would have to be true for the project to be judged a success or a failure. It is deliberately written as a falsifiable statement of intent rather than as promotional material: every claim below is either already demonstrated in the reference implementation, or is explicitly marked as a hypothesis to be tested through implementation, formal analysis, and community review.

## The Central Question

Nearly every general-purpose programming language in production use today represents a program as an ordered sequence of text: a linear stream of tokens parsed into a tree, walked in a well-defined order. This is not an accident — text is easy to store, diff, version, and transmit, and linear control flow maps naturally onto the instruction sequencing of conventional processors. But it is a *choice*, made early in the history of computing and rarely revisited, and it is not the only coherent way to describe computation.

Eye of Horus exists to ask a narrower and more answerable version of a much older question in programming-language research: **what happens to the semantics, the tooling, and the pedagogy of a language if spatial position and geometric relationship are treated as first-class carriers of meaning, rather than as an implementation detail hidden below the abstraction line?**

This is not a new question in the abstract. Dataflow languages, visual programming environments, cellular-automaton-based systems, and a long line of esoteric languages (Befunge's two-dimensional instruction grid is the clearest ancestor) have explored pieces of this space. Eye of Horus's specific contribution is to combine three ideas that are not usually combined in a single system:

1. **Coordinate-bound values.** Every named quantity in the language occupies an explicit point in three-dimensional space, and that point is not cosmetic — it determines where the value is stored (see the phi-pi addressing scheme in [`MEMORY_MODEL.md`](MEMORY_MODEL.md)) and which activation events can reach it.
2. **Field-triggered activation instead of a single instruction pointer.** Execution is modeled as one or more expanding activation fronts ("pulses") that reach and activate geometric structures, rather than as a single program counter stepping through a linear instruction stream. See [`RUNTIME.md`](RUNTIME.md) and the formal semantics in [`spec/SEMANTICS.md`](spec/SEMANTICS.md).
3. **Geometry as a teaching and debugging surface.** Because programs have an inherent spatial layout, a visualizer can render program state as a literal three-dimensional scene rather than as an ASCII call stack, which the project treats as a research opportunity in program comprehension, not merely as a cosmetic feature.

## What Success Looks Like

Eye of Horus will have succeeded in its stated aims if, over the project's lifetime, it produces:

- A **precise, versioned language specification** — lexical grammar, formal grammar, static semantics, and a small-step operational semantics — sufficient for an independent implementer to build a conforming interpreter without consulting the reference source code. Substantial progress toward this already exists in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) and [`spec/SEMANTICS.md`](spec/SEMANTICS.md).
- A **Rust reference implementation** that is small enough to read in full, tested to a standard where every language construct has an executable example, and honest in its own documentation about which parts of the grammar are parsed but not yet given execution semantics.
- **Visual debugging tooling** that renders spatial state, activation fronts, and shape geometry as an inspectable scene, with the explicit research goal of measuring — not merely asserting — whether this improves comprehension relative to conventional textual debuggers.
- **Educational material** — a book, worked examples, and course-style modules — aimed at readers who want to understand alternative computation models, not only readers who want to ship software in the language.
- A **living RFC process** through which syntax, semantics, and standard-library design decisions are proposed, argued, and recorded, so that the language's evolution is auditable rather than ad hoc.
- **Editor and language-server support** sufficient for the language to be usable as more than a specification exercise.
- A **website and reference documentation** that make the above artifacts discoverable to newcomers without requiring them to read the source tree.

## What Success Does Not Require

Eye of Horus does not need to displace, outperform, or compete commercially with general-purpose languages such as Rust, Python, Go, or Zig in order to be a worthwhile project. Its value proposition is research and pedagogical, not industrial. Consequently:

- The project will not publish performance claims without reproducible benchmark methodology (see [`PERFORMANCE.md`](PERFORMANCE.md)).
- The project will not describe the phi-pi memory-addressing scheme as a security mechanism; it is a deterministic, public spatial-hashing function (see [`MEMORY_MODEL.md`](MEMORY_MODEL.md) and [`SECURITY_MODEL.md`](SECURITY_MODEL.md)).
- The project will not assert Turing-completeness, formal type soundness, or any other property that has not been either proved or explicitly cited as an open problem (see the whitepaper's Open Problems section).

## Near-Term Scope

The long-term vision above is intentionally broad; the near-term engineering goal is narrower and is restated here so that contributors and readers can distinguish aspiration from active work: **define the smallest executable language core that can be parsed, simulated, tested, and explained without exaggeration.** Concretely, this means a lexer and parser with full diagnostic coverage, a deterministic pulse-activation runtime for the currently load-bearing instruction subset, a documented and tested value model, and a test corpus that exercises every construct claimed to work. Progress against this near-term goal is tracked in [`ROADMAP.md`](ROADMAP.md), and the boundary between "implemented" and "planned" is maintained explicitly throughout this documentation set — see [`STYLE_GUIDE.md`](STYLE_GUIDE.md) for the editorial rule that enforces this.

## Relationship to the Manifesto and Design Principles

This document answers *what* the project is trying to build and *why* it is worth building. [`MANIFESTO.md`](MANIFESTO.md) states the values that govern *how* the project conducts itself while building it. [`DESIGN_PRINCIPLES.md`](DESIGN_PRINCIPLES.md) translates those values into concrete engineering rules. Readers who want the argument for geometry-native programming at greater length, with related work and open research questions, should consult the [white paper](whitepaper/eye-of-horus-whitepaper.md).
