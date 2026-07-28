# Eye of Horus: A Geometry-Native Programming Language and Pulse-Activation Computational Model

**Author:** Ciprian Stefan Plesca
**Affiliation:** LocalPulse (independent research)
**Status:** Working draft, v0.1
**Repository:** https://github.com/Ciprian-LocalPulse/Eye-of-Horus

## Abstract

We present Eye of Horus, an experimental programming language and virtual
machine in which spatial geometry is the primary organising principle for
both data storage and control flow. Rather than a flat linear address space,
Eye of Horus programs allocate values at points in continuous three-dimensional
space, addressed through a deterministic quantisation lattice derived from
the golden ratio (φ) and π. Execution is driven not by a program counter
alone but by **Higgs pulses** — simulated expanding wavefronts that
*activate* geometric shapes as they propagate outward from a source vertex.
We describe the language's grammar, static semantics, and operational
semantics; present a complete reference implementation in Rust (lexer,
parser, resolver, type-checker, MIR lowering, optimiser, bytecode emitter,
and stack-based virtual machine); and discuss open theoretical questions,
including the unresolved status of Turing-completeness for the current
instruction set. We position this work explicitly as a **research and
pedagogical artifact** exploring an alternative computational metaphor, not
as a production-ready or performance-competitive system.

## 1. Introduction

### 1.1 Motivation

Most general-purpose programming languages inherit, directly or indirectly,
the von Neumann model: a linear, byte-addressed memory and a program counter
that steps through a linear instruction stream. This model is extraordinarily
successful, but it is a *choice*, not a law of computation. Alternative
models — dataflow architectures, cellular automata, spatial computing
substrates, and reaction-diffusion systems in unconventional computing — have
long explored what happens when locality, propagation delay, and spatial
structure are made first-class rather than incidental.

Eye of Horus asks a narrow, concrete question in this space: *what does a
small, statically-typed, compiled language look like if geometric position
is the address space, and causal activation is expressed as a physically-
motivated propagating pulse rather than an implicit call stack?*

### 1.2 Contributions

1. A formally specified grammar and (partial) operational semantics for a
   geometry-native language (§3–§5, and companion documents
   [`spec/LANGUAGE_SPEC.md`](../spec/LANGUAGE_SPEC.md) and
   [`spec/SEMANTICS.md`](../spec/SEMANTICS.md)).
2. A complete, tested, open-source reference implementation: a nine-crate
   Rust workspace implementing the full pipeline from source text to
   VM execution (§6).
3. The **phi-pi addressing model**, a deterministic spatial quantisation
   scheme mapping continuous coordinates to a discrete lattice (§4.2),
   together with an explicit, honest statement of what this model does
   *not* provide (no cryptographic or security properties — §4.2.1).
4. The **Higgs-pulse activation model**, an operational semantics for
   propagation-driven execution, with a proven monotonicity property
   (§5.2) and a documented list of open problems, including
   Turing-completeness (§8).

### 1.3 What this paper is not claiming

We are explicit, in the spirit of honest scholarship, about the limits of
this work:

- We do not claim performance advantages over conventional VMs. No
  benchmarks against production language runtimes are presented; the
  reference implementation has not been profiled or optimised beyond basic
  constant-folding and dead-code elimination (§6.5).
- We do not claim the phi-pi addressing scheme has any relationship to
  cryptographic hashing, security, or privacy properties. It is a
  spatial-hashing convenience for VM memory layout, full stop (§4.2.1).
- We do not claim to have resolved whether the current instruction set is
  Turing-complete. `LOOP`/`JumpIf` constructs parse but are not yet wired
  into VM dispatch as of v0.1 (§8.1); this is an open engineering and
  theoretical question, not a settled result.
- The name "Eye of Horus" and the visual glyph 𓂀 are used as an evocative,
  memorable identity for this project. We make no claims of continuity with,
  authority over, or special insight into any historical Egyptological,
  religious, or esoteric tradition. Readers interested in the historical Eye
  of Horus symbol should consult Egyptological scholarship; this project is
  unrelated to that literature except by name.

## 2. Related work

Eye of Horus draws loose conceptual inspiration from several established
lines of research, without claiming novelty over any of them individually;
its contribution is the specific combination and the concrete language
design, not any single idea in isolation.

- **Dataflow and spatial computing architectures** (e.g. Kahn process
  networks, systolic arrays) demonstrate that computation can be organised
  around spatial/topological structure rather than a linear instruction
  stream.
- **Cellular automata** (Conway's Life, Wolfram's classification of CA
  rule spaces) show that local, propagating update rules over a spatial
  grid can produce rich — in some cases Turing-complete — computational
  behaviour. The Higgs-pulse model's "wavefront activation" is a continuous-
  space analogue of the discrete neighbourhood-activation rules found in CA.
- **Physical/unconventional computing** (reaction-diffusion computers,
  physarum-based computing) explores using literal physical propagation
  phenomena as a computational substrate. Eye of Horus is a *simulated*
  analogue of this idea within a conventional digital VM — we do not claim
  any physical computing substrate is involved.
- **Spatial hashing and locality-sensitive addressing** in computer graphics
  and physics engines (e.g. uniform grids, k-d trees) inform the phi-pi
  addressing scheme's role as a storage-locality mechanism, though our
  specific golden-ratio quantisation choice is aesthetic/exploratory rather
  than derived from an established locality-sensitive hashing family.

## 3. Language overview

### 3.1 Declarations

An Eye of Horus program is a sequence of top-level *items*. The core
declaration forms are:

```eoh
ORIGIN x, y, z                         // establish the coordinate origin
VERTEX name x, y, z                    // bind a name to a spatial point
EDGE from -> to                        // declare a directed edge (metadata)
SHAPE_TETRA name v1, v2, v3, v4         // a tetrahedron over 4 named vertices
PULSE_HIGGS origin, v=velocity          // emit an expanding activation wavefront
LET name = expr                         // bind a computed value
FN name(params) -> ReturnType { ... }   // declare a function
```

The full grammar is given in EBNF in [`spec/LANGUAGE_SPEC.md`](../spec/LANGUAGE_SPEC.md).

### 3.2 A worked example

```eoh
ORIGIN 0.0, 0.0, 0.0

VERTEX A 1.0, 1.0, 1.0
VERTEX B 1.0, -1.0, -1.0
VERTEX C -1.0, 1.0, -1.0
VERTEX D -1.0, -1.0, 1.0

SHAPE_TETRA T1 A, B, C, D

PULSE_HIGGS A, v=1.618
```

This program declares a regular tetrahedron and emits a pulse from vertex
`A` at velocity φ (the golden ratio). Under the operational semantics of
§5, the pulse's wavefront radius grows linearly with simulation tick; once
the radius exceeds the distance from `A` to each of `B`, `C`, `D`, those
vertices become *activated*.

## 4. Spatial memory model

### 4.1 Coordinates

Every spatial value in Eye of Horus is a validated triple `Coord3D { x, y,
z } ∈ ℝ³`, constructed only through a smart constructor that rejects
non-finite values and magnitudes beyond `MAX_COORD` (§`eoh_core::coordinates::Coord3D::new`).
This "parse, don't validate" discipline means invalid coordinates are
unrepresentable anywhere downstream of construction — a standard but
valuable discipline borrowed from robust systems programming practice.

### 4.2 The phi-pi addressing lattice

We define the addressing function:

```
α(x, y, z) = ( round(x/q), round(y/q), round(z/q) ),   q = φ/π ≈ 0.515
```

where φ is the golden ratio `(1+√5)/2` and π is the usual circle constant.
This maps continuous coordinates onto a discrete integer lattice used as the
VM's storage key space (`PhiPiAddress`, `eoh_core::coordinates`).

**Why φ/π specifically?** The choice is deliberately aesthetic and
exploratory rather than derived from an optimality proof. φ is
famously the "most irrational" number in the sense of continued-fraction
approximation (its continued fraction is `[1;1,1,1,...]`), which gives
Fibonacci-lattice-style point distributions good equidistribution
properties in related contexts (e.g. sunflower-seed spiral packings, as
implemented in `eoh_stdlib::math::fibonacci_sphere`). Whether this
equidistribution property meaningfully improves VM storage locality
*for arbitrary Eye of Horus programs* is an open empirical question we
have not yet benchmarked (see §8.3) — the ratio is currently justified by
mathematical elegance and thematic consistency with the project's spiral/
golden-ratio motifs, not by a locality proof.

#### 4.2.1 Explicit non-claims about the phi-pi model

Because "golden ratio" and "phi" sometimes appear in pseudo-cryptographic or
pseudo-scientific contexts, we state plainly: **the phi-pi addressing
function is a public, deterministic, easily-invertible quantisation.** It
provides:

- No confidentiality (any observer with the formula can compute any
  address from any coordinate, and vice versa up to lattice resolution).
- No collision resistance guarantee suitable for security applications
  (nearby points collide *by design* — that is the point of the
  quantisation).
- No relationship to established cryptographic primitives.

It is purely an engineering choice for organising the spatial store, on par
with choosing a hash function for a `HashMap` — nothing more.

### 4.3 The spatial field

`SpatialField<V>` (`eoh_core::field`) is a `HashMap<PhiPiAddress, V>` —
the VM's only storage mechanism. There is no separate "heap" or "stack" for
long-lived data; the operand stack (§6.6) exists purely for evaluating
expressions and is not directly observable as program state.

## 5. The Higgs-pulse activation model

### 5.1 Definition

A pulse `p = ⟨origin, velocity, birth_tick⟩` defines a wavefront whose
radius at simulation tick `t` is:

```
radius(p, t) = max(0, t − birth_tick) · velocity
```

A point is *activated* by `p` at tick `t` iff its Euclidean distance from
`origin` is at most `radius(p, t)`. An `ActivationField` is a set of pulses;
a point is active under the field if any member pulse activates it.

### 5.2 Monotonicity

**Proposition.** For fixed `p` and point `x`, if `activates(p, x, t)` holds
for some `t`, then `activates(p, x, t')` holds for all `t' ≥ t`, provided
`velocity ≥ 0`.

*Proof sketch.* `radius(p, ·)` is non-decreasing in its tick argument when
`velocity ≥ 0`, since it is an affine function of `max(0, t − birth_tick)`,
itself non-decreasing. Since `activates` is a fixed threshold test against a
non-decreasing quantity, once satisfied it remains satisfied. ∎

This gives Eye of Horus programs a form of monotonic "unlocking" semantics:
once a pulse reaches a shape, that shape stays reachable for the remainder
of the simulation (in the current v0.1 semantics — see §8.2 for discussion
of bounded-duration pulses as a future relaxation of this property).

### 5.3 Why "Higgs"?

The name is an evocative borrowing from particle physics' Higgs field —
a field that permeates space and with which other fields interact to
acquire mass/behaviour — used here purely as a metaphor for "a field that
activates things when they interact with it." We make no claim of physical
correspondence to the Higgs mechanism in the Standard Model; this is a
naming choice for a software artifact, not a physics claim.

## 6. Reference implementation

The reference implementation is a nine-crate Cargo workspace:

| Crate | Responsibility |
|---|---|
| `eoh-core` | `Coord3D`, `PhiPiAddress`, `Pulse`, `ActivationField`, `SpatialField<V>`, error types |
| `eoh-lexer` | Hand-written tokeniser, no external parser-generator dependency |
| `eoh-ast` | `serde`-serialisable AST node definitions |
| `eoh-parser` | Recursive-descent parser with standard precedence climbing for expressions |
| `eoh-compiler` | Resolver → structural type-checker → MIR lowering → constant-folding/DCE optimiser → bytecode emitter |
| `eoh-vm` | Stack-based bytecode interpreter over the spatial field |
| `eoh-stdlib` | Geometry (centroid, bounding box, tetrahedron surface area), math (lerp, clamp, golden angle, Fibonacci-sphere), and spatial-query (nearest-vertex, radius-query) utilities |
| `eoh-lsp` | Scaffolded Language Server Protocol implementation (capabilities/diagnostics/handlers modules present; event loop not yet wired — Phase 3) |
| `eoh-cli` | The `eoh` command-line tool (`check`, `build`, `run`, `lex`, `ast`, `status`, `version`) |

### 6.1–6.6 Pipeline stages

The compiler pipeline (`eoh_compiler::compile`) runs, in order: lexing,
parsing, name resolution, structural type-checking, MIR lowering (AST →
flat instruction sequence), optional optimisation (constant folding at
level ≥1, dead-code elimination at level ≥2), and bytecode emission. Each
stage is independently unit-tested (35 tests pass across the workspace as
of this writing — verified by `cargo test --workspace` in this
repository's CI).

## 7. Worked semantics example

Consider compiling `VERTEX A -1.0, 2.0, 3.0`. The parser produces a
`VertexDecl` whose `x` field is `UnOp(Neg, Float(1.0))` (recall from
§`spec/LANGUAGE_SPEC.md` §2.3 that negative literals are not a lexical
class — `-` is always the `Minus` operator token). MIR lowering translates
unary negation as `PushFloat(0.0); <lower inner>; Sub`, giving the
instruction sequence `PushFloat(0.0), PushFloat(1.0), Sub, PushFloat(2.0),
PushFloat(3.0), DeclareVertex("A")`. With constant folding enabled (`-O1`),
this collapses to `PushFloat(-1.0), PushFloat(2.0), PushFloat(3.0),
DeclareVertex("A")` — verified empirically against the reference
implementation's bytecode output.

## 8. Open problems

We list these explicitly, in the interest of honest research communication,
rather than glossing over unresolved questions.

### 8.1 Turing-completeness is unresolved

The grammar includes `IF`/`ELSE`, `LOOP`, `BREAK`, and `CONTINUE`, and these
parse into AST nodes today. However, MIR lowering and VM dispatch for
conditional branches and loops are **not yet implemented** as of v0.1 (the
lowering pass explicitly returns `NotImplemented` for unhandled expression
forms, and loop/branch statements are currently no-ops in `lower.rs`). Until
these are wired up, we make **no claim whatsoever** about the computational
power of the executable subset of the language — it is presently closer to
a straight-line arithmetic/declaration DSL than a general-purpose language.
Establishing Turing-completeness (or a proof of a strictly weaker
complexity class) once control flow lands is listed as a first-class open
problem, not an assumed conclusion.

### 8.2 Bounded-duration and directional pulses

The monotonicity property of §5.2 is a direct consequence of the
"pulses never shrink" simplification. A more physically evocative model
might support pulse decay, directional (non-isotropic) propagation using
the already-defined but currently-unused `PulseVector` direction field, and
pulse cancellation/interference between multiple sources. None of these are
implemented; `PulseVector::ISOTROPIC` is the only mode exercised by the
current VM.

### 8.3 Locality claims are unverified

We have not benchmarked whether the phi-pi lattice's equidistribution
properties translate into measurable cache-locality or lookup-performance
benefits for `SpatialField<V>` versus a naïve coordinate-tuple hash. This is
planned future work, not a claimed result.

### 8.4 Formal type soundness

The current type-checker performs structural checks (vertex counts, pulse
origin existence) rather than a full soundness-proven type system. A
progress-and-preservation style soundness proof for a future, fuller type
system (generics, shape-kind polymorphism) is unstarted.

### 8.5 Concurrency and multiple simultaneous pulses

The `ActivationField` already supports multiple simultaneous pulses as a
set union (§5.1), but the VM's execution model is single-threaded and
sequential; whether a genuinely concurrent execution model (where
activated shapes execute "simultaneously" once reached) is desirable or
implementable is unexplored.

## 9. Reproducibility

All claims about implementation status, test coverage, and example program
behaviour in this paper are directly checkable against the repository:

```bash
git clone https://github.com/Ciprian-LocalPulse/Eye-of-Horus.git
cd Eye-of-Horus
cargo test --workspace     # 35 tests, 0 failures, as of this writing
cargo run -p eoh-cli -- run examples/01_tetrahedron.eoh
cargo run -p eoh-cli -- status
```

## 10. Conclusion

Eye of Horus is a small, honestly-scoped exploration of what a
geometry-native, pulse-activated computational model might look like as an
actual, runnable language rather than a purely conceptual sketch. Its value,
at this stage, is as a well-specified, well-tested starting point for
further research into spatial computation metaphors — not as a claim of
practical superiority over conventional languages. We invite scrutiny,
replication, and extension via the issue tracker and RFC process described
in [`CONTRIBUTING.md`](../CONTRIBUTING.md).

## Acknowledgements

This work was designed and implemented by the repository author as an
independent research project. Conceptual inspiration is drawn broadly from
the dataflow-computing, cellular-automata, and unconventional-computing
literatures cited informally in §2; no formal collaboration with those
communities is claimed.

## Appendix A: Glossary

| Term | Definition |
|---|---|
| **Phi-pi address** | The result of applying `α(x,y,z) = (round(x/q), round(y/q), round(z/q))`, `q=φ/π`, to a coordinate. |
| **Higgs pulse** | A simulated expanding spherical wavefront from a vertex, used to drive activation. |
| **Activation field** | The union of all live pulses in the current VM state. |
| **Spatial field** | The VM's sole storage mechanism, keyed by phi-pi address. |
| **MIR** | Mid-level Intermediate Representation — the flat instruction sequence produced by AST lowering, prior to optimisation and final bytecode emission. |
