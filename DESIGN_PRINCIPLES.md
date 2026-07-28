# Design Principles

This document translates the values stated in [`MANIFESTO.md`](MANIFESTO.md) into concrete, checkable engineering rules. Each principle below is written so that a reviewer can look at a pull request, a documentation change, or an RFC and determine whether it complies, rather than needing to interpret intent.

## 1. Honesty Before Spectacle

Every document, code comment, release note, and public-facing description in this project must distinguish, at the point where a reader would naturally ask, between:

- what is **implemented and covered by tests today**;
- what is **planned** for a specific, tracked roadmap phase;
- what is a **research question** with no known answer.

A pull request that presents planned behavior as though it already works — in prose, in an example marked as runnable, or in a benchmark table — fails review regardless of the quality of the underlying idea. This is the single most load-bearing principle in the project and is the one most directly inherited from the manifesto's commitment to honesty over hype.

## 2. Specification Before Stabilization

Language surface area (syntax, keyword semantics, the value model, the memory model, the standard library's public interface) is not considered stable until it has a corresponding entry in the formal specification (see [`spec/`](spec/README.md)) and has passed through the RFC process described in [`RFC_PROCESS.md`](RFC_PROCESS.md). Code that implements a feature ahead of its specification is acceptable as an experiment, but must be labeled as experimental in both the code and its accompanying documentation until the specification catches up.

## 3. Tests Before Claims

No document in this repository may assert that a construct "works" unless that construct is exercised by an automated test in the reference implementation's test suite. This applies equally to prose documentation, whitepaper text, and README examples. Where an example is illustrative of *future* syntax rather than a description of tested behavior, it must say so explicitly (see the convention already used in [`README.md`](README.md) and the language book's introduction).

## 4. Geometry as Semantics, Not Decoration

Spatial concepts — coordinates, vertices, shapes, fields, pulses — must carry defined operational meaning in the language's semantics before they are used in illustrative examples, tooling, or marketing-adjacent material such as the repository banner. It is not acceptable for a geometric primitive to exist purely as visual flavor with no corresponding entry in [`spec/SEMANTICS.md`](spec/SEMANTICS.md) or [`SEMANTICS.md`](SEMANTICS.md). Conversely, once a geometric primitive does carry defined semantics, its documentation should explain that semantics precisely rather than relying on the reader's intuition about what the metaphor "probably" means.

## 5. A Rust Implementation With Explicit Safety Boundaries

The reference implementation is written in Rust specifically to obtain memory safety, strong tooling (`cargo fmt`, `clippy`, the test harness), and a performance profile suitable for an interpreter and, eventually, a geometric runtime. This choice does not by itself make the language or its interpreter secure against untrusted input; see [`SECURITY_MODEL.md`](SECURITY_MODEL.md) for the explicit boundary between "the implementation language is memory-safe" and "the interpreter safely executes untrusted programs," which are different and currently unequal claims.

## 6. Community Design Through RFCs

Decisions with long-term consequences for the language — syntax, semantics, the memory model, runtime scheduling behavior, standard-library shape, and compatibility policy — are made through the RFC process, not through unilateral commits, regardless of who authors the change. This principle exists to keep the language's evolution auditable: a future contributor or researcher should be able to reconstruct *why* a given design decision was made by reading its RFC, in the same way RFC 0001 documents the reasoning, tradeoffs, and open questions behind phi-pi addressing.

## 7. Small, Independently Testable Units

Both the Rust implementation and the specification are built as small, composable units — individual crates, individual specification sections, individual RFCs — each of which can be reviewed, tested, and reasoned about on its own. Large, monolithic changes that bundle unrelated design decisions together are discouraged because they make it difficult to apply Principles 1 through 3 to any single part of the change.

## 8. Documentation Is Part of the Deliverable, Not an Afterthought

A feature is not complete when its code compiles and its tests pass; it is complete when its corresponding specification section, user-facing documentation, and — where relevant — RFC have been updated to match. Documentation debt is tracked with the same seriousness as code debt and is visible in [`ROADMAP.md`](ROADMAP.md) and [`CHANGELOG.md`](CHANGELOG.md).
