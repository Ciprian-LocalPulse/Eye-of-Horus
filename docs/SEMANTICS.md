# Semantics

## Purpose

This document is a conceptual map of Eye of Horus's execution semantics, written for a reader who wants the vocabulary and the shape of the model before reading the fully formal small-step semantics in [`spec/SEMANTICS.md`](spec/SEMANTICS.md). It intentionally repeats no proofs or inference rules; those live in the normative document.

## Core Concepts

Eye of Horus semantics map spatial declarations to activation events through the following concepts, listed in the order a program typically introduces them:

- **origin** — the fixed reference point of a program's coordinate space, declared with `ORIGIN`.
- **coordinate** — a validated point in ℝ³, finite and bounded by `MAX_COORD` (see [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §4.4).
- **vertex** — a named coordinate, introduced with `VERTEX`, which becomes both a geometric point and a storage location once it is written to (see [`MEMORY_MODEL.md`](MEMORY_MODEL.md)).
- **vector** — a directed relationship between two named points, introduced with constructs such as `EDGE` or `VECTOR FLOW`.
- **shape** — a geometric aggregate over a fixed or parameterized set of vertices (`SHAPE_TETRA`, `SHAPE_CUBE`, `SHAPE_ICOSA`, `SHAPE_SPHERE`), subject to the arity constraints in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §4.2.
- **pulse** — an expanding activation front emitted from an origin point at a given velocity, introduced with `PULSE_HIGGS`. See [`RUNTIME.md`](RUNTIME.md) for the engineering view of pulse scheduling.
- **activation event** — the moment at which a pulse's wavefront radius first reaches a given point, formally `distance(origin, point) <= radius(t)`.
- **intersection event** — the moment at which two or more geometric structures (shapes, or the activation fields of two pulses) occupy or reach a shared coordinate.

## What Is Defined Today

The operational meaning of `ORIGIN`, `VERTEX`, `EDGE`, the `SHAPE_*` family (subject to arity checks), `LET`, `FN`, arithmetic expressions, and `PULSE_HIGGS`-driven isotropic pulse propagation is defined in full in [`spec/SEMANTICS.md`](spec/SEMANTICS.md), including:

- **value encoding** — the `Value` domain (`Float`, `Bool`, `Str`, `Coord`, `Unit`) and the partial map from lattice address to value that constitutes the spatial field;
- **evaluation order** — a strict left-to-right, stack-based evaluation for arithmetic and calls, matching the bytecode instruction set;
- **mutation and data flow** — `Store` and `Load`, indexed indirectly through the vertex-name-to-coordinate table;
- **termination** — the `Halt` instruction and terminal configuration; general termination for programs using `LOOP` is explicitly an open problem, not a proven property (see below).

## What Is Not Yet Defined

- **Diagnostics for ambiguous geometry** beyond the arity checks already specified — for example, what should happen when two declared shapes overlap in a way that is not a simple vertex-count violation is not yet fully worked out.
- **General termination and confluence** for programs that use `LOOP` and `JumpIf` once those constructs are fully wired into the runtime (tracked as a Phase 1 item in [`ROADMAP.md`](ROADMAP.md)).
- **Bounded-duration and directional pulses**, and pulse interference or cancellation, which the white paper discusses as future extensions but which are not load-bearing in the current VM.

## Known Semantic Rough Edges

[`spec/SEMANTICS.md`](spec/SEMANTICS.md) §5 documents, deliberately and by name, two rough edges in the current reference semantics: the default-to-origin resolution of `Load`/`Store` for an unbound name (rather than a runtime fault), and the conflation of simulated pulse time with raw instruction count. These are recorded here as well because they are exactly the kind of detail that a summary document could otherwise be tempted to smooth over; per [Design Principle 1](DESIGN_PRINCIPLES.md#1-honesty-before-spectacle), they are not.

## Where to Go Next

Readers who want the formal judgments should read [`spec/SEMANTICS.md`](spec/SEMANTICS.md) directly. Readers who want the engineering perspective on how these semantics are scheduled and executed should read [`RUNTIME.md`](RUNTIME.md). Readers who want the broader research motivation for the pulse-activation model, including why it is named after the Higgs field, should read the [white paper](whitepaper/eye-of-horus-whitepaper.md), §5.3.
