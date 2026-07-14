# EYE OF HORUS (EoH)
## A Geometry-Native, Field-Triggered Programming Language

**Specification, Verified Reference Implementation, and Research Roadmap**

**Author:** Ciprian Stefan Plesca
**Affiliation:** Founder, AgentFlow Enterprise — [agentflow-enterprise.com](https://agentflow-enterprise.com)
**Repository:** [github.com/Ciprian-LocalPulse/Eye-of-Horus](https://github.com/Ciprian-LocalPulse/Eye-of-Horus)
**Version:** 1.0
**Date:** July 2026
**Reference Commit:** `8259d04` (July 13, 2026)
**Status:** Public Research Whitepaper — supersedes the v0.1 internal draft
**License:** Apache License 2.0 (free and open source, in perpetuity)

---

## Table of Contents

1. Abstract
2. Introduction & Motivation
   - 2.1 Scope and Non-Goals
3. Positioning: Eye of Horus in the Lineage of Non-Linear Languages
4. Design Philosophy
5. Language Specification
   - 5.1 Syntax
   - 5.2 Draft Formal Grammar (EBNF)
   - 5.3 Semantics
   - 5.4 Instruction Set
6. Memory Architecture — The φ·π Addressing Scheme
7. Execution Model — The Higgs Pulse Engine
8. Implementation Status & Verification
   - 8.1 What Exists Today
   - 8.2 Verified Build & Test Evidence
   - 8.3 Reference Architecture (Planned Pipeline)
9. Open Design Questions
10. Applications & Research Value
11. Roadmap
12. Governance, Licensing & Community Support
13. Security Posture
14. Conclusion
15. About the Founder
16. How to Cite
17. References

---

## 1. Abstract

For seven decades, mainstream programming languages have inherited a single structural assumption from the Turing machine: that a program is a sequence — a one-dimensional tape of instructions read in order, left to right, top to bottom. **Eye of Horus (EoH)** proposes to break that assumption. In EoH, a program is not written — it is *built*. Instructions exist as geometric solids positioned in three-dimensional space; they remain computationally inert until struck by a simulated field of execution radiating outward from a fixed origin; and they are addressed in memory through a scheme derived from two irrational constants, the golden ratio (φ) and π.

This whitepaper defines EoH's philosophy, formal syntax, semantics, instruction set, memory model, and execution architecture; documents the independently verified state of its open-source Rust reference implementation as of commit `8259d04` (July 13, 2026); situates the project within the decades-long, documented tradition of esoteric and spatial programming languages; and lays out a phased roadmap. Eye of Horus is released under the **Apache License 2.0** and is community-funded. It is not proposed as a replacement for Python, Rust, or any production language. Its value lies in what it teaches, what it makes visible, and the research questions it opens in programming-language theory, computational art, and spatial computing.

## 2. Introduction & Motivation

Since Alan Turing's 1936 formalization of computation, and John von Neumann's architecture for realizing it in hardware, the overwhelming majority of programming languages have modeled execution as a strictly ordered sequence of steps. This assumption is so deeply embedded that most programmers never question it — a program *is* a list of instructions, executed one after another.

That assumption is a design choice, not a law of nature. Alternative computational models are well studied — dataflow programming, cellular automata, and spatial or visual programming all reject strict sequentiality in one way or another. Eye of Horus approaches this space from an unusual angle: rather than starting from computer science and generalizing outward, it starts from geometry, physical metaphor, and mathematical proportion, and asks what a language would look like if it were designed by someone thinking about *space, mass, and proportion* first, and *instructions* second.

The result is a language in which:

- **State is spatial.** A value is not stored in a register; it exists as a point in three-dimensional space.
- **Execution is physical.** Nothing runs until it is "found" by a propagating field, modeled loosely on how the Higgs field is popularly — if reductively — described as giving mass to other particles.
- **Structure is irrational, by design.** Memory addressing and directional flow are governed by φ and π: constants chosen because they are non-repeating, non-terminating, and central to how proportion recurs throughout mathematics and nature.

### 2.1 Scope and Non-Goals

> **This project is intentionally honest about its maturity, in this document and throughout its repository.** Eye of Horus is not a finished language, not a production compiler, and not a scientifically validated performance or security breakthrough. It is a public specification, an active research program, and an early Rust implementation scaffold.

To keep this proposal reviewable, it states plainly what EoH does **not** claim:

- EoH does **not** claim to outperform, or to be a production substitute for, conventional languages such as Python, Rust, C++, or JavaScript.
- EoH's φ·π memory addressing scheme is **not** a cryptographic security mechanism. Because the address function is deterministic and published as open source, it offers no obscurity-based protection — discussed in Section 6 and reaffirmed in Section 13.
- EoH does **not** claim to implement, simulate, or provide a technical bridge to quantum computation. Any connection drawn between EoH and "quantum-style thinking" is pedagogical, not technical (Section 10).
- EoH's Turing-completeness has **not** yet been formally established (Section 9).
- No type system, parser, or runtime is implemented yet (Section 8). Where this document describes such components, it is describing a specification target, clearly marked as planned.

## 3. Positioning: Eye of Horus in the Lineage of Non-Linear Languages

EoH is not the first language to challenge linear execution, and it is not presented as though it were. Esoteric programming languages ("esolangs") — languages deliberately built to probe the outer edges of what a language can be, whether as a proof of concept, a piece of software art, or a joke — form a documented, decades-old field with an active research and hobbyist community [1]. The primary community wiki alone documents more than 7,000 distinct languages, contributed by upward of 9,000 registered users since 2005 [3]. EoH joins a large, serious, and long-running body of work; it does not invent the category.

Three prior languages are directly relevant to EoH's design and are acknowledged explicitly:

- **Befunge (1993, Chris Pressey).** Befunge broke from linear execution by giving its instruction pointer a two-dimensional grid to move across, rather than a single line to follow [2]. EoH is best understood as Befunge's idea taken one dimension further and inverted: instead of a pointer that *visits* fixed code on a 2D grid, EoH uses a field that *discovers* dormant code floating in continuous 3D space.
- **Malbolge (1998).** Malbolge was designed to be nearly unusable by a human programmer. Notably, its ability to run a non-terminating loop went undemonstrated for roughly seven years after its creation, only being proven in 2005 [1]. This is directly relevant precedent for EoH's own open questions around Turing-completeness (Section 9): in the esolang tradition, it is normal for a language's formal computational power to be established well after its initial release, not before.
- **Piet (David Morgan-Mar).** Piet encodes a program as an abstract image inspired by the paintings of Piet Mondrian, where color and position — not text — determine control flow, directly anticipating EoH's premise that a program can be a spatial object rather than a string of characters.

EoH's specific contribution to this lineage is threefold: (1) execution triggered by a propagating field rather than a moving pointer, (2) a memory model derived from irrational mathematical constants rather than arbitrary or thematic encoding, and (3) continuous 3D geometric primitives — points, vectors, solids — rather than a discrete grid or 2D image. To the author's knowledge, this specific combination does not appear elsewhere in the documented esolang corpus, though no claim of absolute uniqueness is made; the field is too large and active for any single contributor to survey exhaustively.

## 4. Design Philosophy

EoH rests on three tenets, stated in the project's own repository as design principles:

**I. Code is Space.** The programmer does not write algorithms; they construct geometric arrangements inside a three-dimensional void. A variable is a coordinate. An operation is a solid. A program is, quite literally, a small sculpture. Code can be spatial without pretending that text no longer matters.

**II. Code is Mass, Activated by a Field.** No instruction executes simply because it appears in the source file. Every declared shape begins inert — "massless," in the language's own vocabulary — until it is struck by a spherical wavefront expanding outward from a defined origin. This is a *metaphor*, deliberately borrowed from how the Higgs boson — popularly, if reductively, nicknamed the "God particle" — is often described as giving other particles their mass. EoH does not simulate particle physics, and no claim of physical accuracy is made. Geometry should clarify computation, not obscure it.

**III. Structure Should Be Irrational, on Purpose.** Most languages optimize for predictable, discrete structure: addresses increment by one, loops count in whole numbers. EoH deliberately does the opposite, using φ and π — constants that are provably irrational and recur throughout natural growth patterns and geometry — to govern memory layout and directional flow. Research claims about this choice must be kept separate from implementation facts, which is why Sections 6 and 13 state plainly what this scheme does *not* provide.

## 5. Language Specification

### 5.1 Syntax

EoH source files use the `.eoh` extension and are plain text. A file is a declarative list of coordinates, geometric primitives, and a single execution trigger:

```
// Define the coordinate origin
ORIGIN 0.0, 0.0, 0.0

// Declare points in space
VERTEX A 1.0, 0.0, 0.0
VERTEX B 0.0, 1.0, 0.0
VERTEX C 0.5, 0.5, 0.0

// Declare geometric primitives (instructions)
SHAPE_TETRA T1 A, B, C, ORIGIN     // an arithmetic operator
SHAPE_CUBE OUT C, size=0.2          // an I/O portal

// Trigger execution
PULSE_HIGGS ORIGIN, v=1.0
```

There is no `if`, `while`, or `print` keyword. Control flow, iteration, and output are all expressed geometrically (Section 5.4). This syntax is illustrative and subject to RFC review; it is not yet parsed by a complete compiler (Section 8).

### 5.2 Draft Formal Grammar (EBNF)

A first-pass grammar skeleton is maintained in the repository at `spec/grammar.ebnf`. It is reproduced here for reference and is explicitly marked, in the source file itself, as incomplete:

```ebnf
(* Eye of Horus grammar draft placeholder. *)

program     = { declaration } , pulse ;
declaration = origin | vertex | vector | shape ;
origin      = "ORIGIN" , number , "," , number , "," , number ;
vertex      = "VERTEX" , identifier , number , "," , number , "," , number ;
vector      = "VECTOR" , identifier , identifier , "->" , identifier ;
shape       = shape_kind , identifier , identifier_list ;
pulse       = "PULSE_HIGGS" , identifier , "," , "v" , "=" , number ;

(* TODO: define lexical rules, comments, whitespace, literals, and errors. *)
```

Lexical rules — whitespace handling, comment syntax, numeric literal formats, and error recovery — are an explicit gap and are tracked as a Phase 1 deliverable (Section 11), to be closed through the project's RFC process (Section 12) rather than decided unilaterally.

### 5.3 Semantics

- **State.** A variable does not live in a register or stack slot; it *is* a coordinate `(x, y, z)` in the program's spatial matrix.
- **Mass.** When a solid — a tetrahedron, for instance — is declared, the interpreter computes its volume. Volume is mass. Higher-mass instructions are architecturally "heavier": they are defined to consume more of the field's energy and, in the reference implementation, more execution cycles.
- **Intersection.** When two geometric primitives overlap in space, that overlap constitutes a data-transfer event — the value carried by one shape becomes available to the other. This is EoH's substitute for function calls and variable assignment between operations.

### 5.4 Instruction Set

Each geometric primitive corresponds to a class of conventional operation:

| Primitive | Classical Equivalent | Function in EoH |
|---|---|---|
| **VERTEX** (point) | numeric literal | Declares a constant or raw value at a spatial coordinate |
| **VECTOR** (line) | pointer / assignment | Transfers data between two points; encodes direction of flow |
| **TETRAHEDRON** | `+ − × ÷` | Performs an arithmetic operation; which operator applies is a function of the solid's angle of inclination relative to the origin |
| **SPHERE** | `while` / `for` | A gravitational loop: execution is pulled back to the sphere's center repeatedly until its radius (the loop's condition variable) reaches zero |
| **CUBE** | `print()` / `input()` | An input/output portal; any data intersecting the cube is displayed to, or read from, the user |

## 6. Memory Architecture — The φ·π Addressing Scheme

Conventional memory is sequential: address 0, then 1, then 2, incrementing by a fixed step. EoH departs from this by computing the memory address of the *n*-th declared entity as:

```
M(n) = floor( n × φ × π )

where φ = (1 + √5) / 2 ≈ 1.6180339887…   (the golden ratio)
      π ≈ 3.14159265…
```

This produces a deterministic, monotonically increasing, but non-uniformly-spaced address sequence, loosely inspired by the growth pattern of golden and Fibonacci spirals — a **stylized addressing aesthetic**, not a literal spiral geometry, and not a cryptographic mechanism. Because `M(n)` is a fixed, published formula — EoH being fully open source under Apache-2.0 — it offers **no obscurity-based security benefit**: any party with the source code, which by design is everyone, can compute any address in constant time. The project's own `SECURITY_MODEL.md` states this directly: *the phi-pi memory model is not a security feature.* This whitepaper affirms the same position rather than softening it for marketing purposes.

The scheme's genuine research value lies elsewhere:

- It provides a natural, non-adjacent memory layout that is a useful small-scale sandbox for studying cache behavior, hashing, and non-linear data structures.
- It gives the language a distinct, irrational "texture" directly legible in how programs are laid out spatially — closer in spirit to generative art than to systems programming.
- It creates a concrete, classroom-usable example of *why* security through obscurity fails even when the obscuring function looks mathematically sophisticated — a genuinely useful teaching point independent of EoH itself.

## 7. Execution Model — The Higgs Pulse Engine

EoH has no linear instruction pointer. Instead, at the command `PULSE_HIGGS`, a spherical wavefront expands outward from the origin at a configurable velocity `v`. On each simulation tick, the engine measures the Euclidean distance from the origin to the center of every still-dormant shape.

This distance computation is implemented and tested in the reference codebase today:

```rust
impl Point3 {
    /// Computes Euclidean distance to another point.
    pub fn distance_to(self, other: Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
```

This function is covered by an automated test verifying, for example, that the distance between `(0, 0, 0)` and `(3, 4, 12)` is exactly `13.0` — a direct extension of the 3-4-5 right triangle into three dimensions (Section 8.2).

The full pulse engine that will consume this primitive — expanding a wavefront, scanning dormant shapes, and dispatching activations — is specified but **not yet implemented**. The intended design, in Rust-flavored pseudocode:

```rust
fn pulse_higgs_engine(origin: Point3, speed: f64, shapes: &mut Vec<Shape>) {
    let mut radius = 0.0;
    while radius < MAX_RADIUS {
        for shape in shapes.iter_mut() {
            if !shape.activated {
                let d = origin.distance_to(shape.center);
                if radius >= d {
                    execute_instruction(shape);
                    shape.activated = true;
                }
            }
        }
        radius += speed;
    }
}
```

**A design property worth flagging honestly:** because activation depends only on distance from the origin, shapes that happen to be exactly equidistant are struck within the same simulation tick. This is an architectural *opportunity* for natural concurrency — a full implementation could dispatch same-tick shapes to parallel worker threads — but it is not, at this stage, a demonstrated or benchmarked performance property. It is a Phase 1/2 research question (Section 9), not a completed result.

## 8. Implementation Status & Verification

### 8.1 What Exists Today

Unlike many early-stage language proposals, every claim of "implemented" in this document is backed by source code and passing tests, not aspiration alone. As of commit `8259d04` (July 13, 2026), the reference implementation is a two-crate Rust workspace:

| Component | Purpose | Status |
|---|---|---|
| `eoh-core` | `Point3` geometry, Euclidean distance, the φ·π address function | **Implemented, tested** |
| `eoh-cli` | Command-line status reporter | **Implemented (minimal)** |
| `eoh-parser` *(planned)* | Lexer, parser, AST, diagnostics | Not started |
| `eoh-runtime` *(planned)* | Pulse simulation, deterministic event scheduling | Not started |
| VS Code extension | Syntax-highlighting manifest only | Placeholder |
| Language server (LSP) | Design notes only | Placeholder |

The workspace totals roughly 95 lines of Rust across five files. This is disclosed intentionally — the project's own README describes it as "a minimal scaffold," not an oversight, and this document adopts the same framing rather than inflating it.

### 8.2 Verified Build & Test Evidence

The following was independently reproduced from a clean clone of the repository, in the interest of making this whitepaper's claims falsifiable rather than promotional:

```
$ cargo build --workspace
   Compiling eoh-core v0.1.0
   Compiling eoh-cli v0.1.0
    Finished dev [unoptimized + debuginfo] target(s)

$ cargo test --workspace
running 2 tests
test tests::distance_uses_euclidean_geometry ... ok
test tests::phi_pi_address_is_monotonic_for_initial_range ... ok
test result: ok. 2 passed; 0 failed; 0 ignored

$ cargo run -p eoh-cli -- --status
Eye of Horus - research draft: specification scaffold and minimal Rust utilities only
Example phi-pi address M(1): 5
```

Zero build warnings, zero failing tests, one working binary. The scope is small on purpose, and the project's stated intent is to keep it small until each addition earns its place with a corresponding test.

### 8.3 Reference Architecture (Planned Pipeline)

```
 .eoh source file
        |
        v
 Lexer / Parser              (planned: eoh-parser)
        |
        v
 Spatial Matrix Builder      (planned)
        |
        v
 Volume & Intersection Engine (planned)
        |
        v
 Higgs Pulse Generator  ------>  Execution Queue  ------>  Output   (planned: eoh-runtime)
```

Today, only the geometric primitives at the foundation of this pipeline (`Point3`, distance, and address calculation) exist in `eoh-core`. Everything above the first layer is a specification target, not running code.

## 9. Open Design Questions

Consistent with the project's own documentation (`TYPE_SYSTEM.md`, `SECURITY_MODEL.md`), several foundational questions remain open and are treated as research milestones rather than settled facts:

**Semantics and execution**

1. **Value encoding.** The exact rule by which a coordinate "carries" a scalar value, as opposed to merely a position, needs a precise, testable specification.
2. **Simultaneous collision resolution.** When multiple shapes are struck in the same tick, a deterministic execution order — or explicit parallel semantics — must be defined.
3. **Computational power.** Whether EoH, as specified, is Turing-complete is unresolved. As Section 3 notes, this is normal for a new esoteric language — Malbolge's own Turing-completeness took years to establish — but it is treated as unproven, not assumed.

**Type system**

4. Are coordinates themselves values, references, or both?
5. Are geometric primitives typed by dimension, and does the language need distinct scalar, vector, matrix, field, and shape kinds?
6. Should type inference exist in early versions, or should all types be explicit until the semantics stabilize?

Future RFCs (Section 12) are expected to resolve these questions before syntax or semantics are declared stable, and before further example programs are published as anything other than illustrative placeholders.

## 10. Applications & Research Value

EoH is proposed honestly as a research, educational, and artistic contribution — not as infrastructure for production software. Within that scope, several concrete application areas justify continued development:

**Programming-language theory & education.** A hands-on, buildable artifact for teaching alternative computation models — useful in university PL-theory electives, esolang community events, and as a live demonstration of why the sequential Turing-machine model is a choice rather than a necessity.

**Computational art & live coding.** EoH's execution model is inherently visual — a 3D scene, activated wave by wave — making it naturally suited to generative art and live-coding performance, in the tradition of tools like Processing and TidalCycles.

**Spatial computing & HCI research.** As AR/VR and spatial-computing interfaces mature, languages that natively reason about 3D placement and physical metaphors may offer a useful design vocabulary for future spatial-programming environments. This is presented as a plausible, forward-looking research direction, not a proven capability.

**Security pedagogy, reframed honestly.** As discussed in Sections 6 and 13, the φ·π addressing scheme is not a security mechanism. Its genuine value is as a teaching example of why deterministic-but-public obfuscation fails to provide real security — motivating discussion of genuinely secure alternatives such as capability-based memory models.

**"Quantum-style" intuition building, reframed honestly.** Representing state as continuous spatial position, rather than as a discrete binary value, exercises some of the same non-binary intuition that probabilistic and vector-space reasoning require. EoH makes no claim of implementing or approximating quantum computation.

**Community & developer visibility.** Like Piet, Malbolge, and Brainfuck before it, a well-executed esoteric language can become a distinctive, widely shared engineering showcase — independent of any production use case.

## 11. Roadmap

The roadmap below mirrors the phases tracked in the repository's own `ROADMAP.md`, so that this whitepaper and the project's living planning document never drift apart.

| Phase | Focus | Key Deliverables | Status |
|---|---|---|---|
| **0 — Public Research Repository** | Foundation | Repository structure, Apache-2.0 license, governance and contribution documents, whitepaper, specification skeleton, minimal Rust workspace | **In progress** |
| **1 — Language Core** | The actual interpreter | Lexer, parser, AST, diagnostics, spatial matrix representation, deterministic pulse simulation, resolved value-encoding rules, first genuinely executable `.eoh` examples, parser/semantics test suite | Planned |
| **2 — Tooling Alpha** | Usability | CLI interpreter prototype, visualizer prototype, VS Code syntax highlighting, initial LSP diagnostics, RFC-backed syntax stabilization | Planned |
| **3 — Research & Community** | Growth | Formal semantics review, computational-power analysis (Turing-completeness), educational materials, standard-library experiments, community working groups | Planned |

Dates are intentionally omitted, consistent with the project's own roadmap, until maintainers can commit to a schedule responsibly rather than publish estimates likely to be revised.

## 12. Governance, Licensing & Community Support

**License.** Eye of Horus is released under the **Apache License 2.0**, chosen over more permissive alternatives specifically for its explicit patent grant and contributor-protection terms — a deliberate choice for a project whose founder intends to invite outside contributors and RFC-driven design input from the outset. The license is free for any individual, educator, or organization to use, study, modify, and redistribute, in perpetuity.

**Governance.** Eye of Horus begins as a founder-led research project. Ciprian Stefan Plesca is the founding author, original architect of the language's philosophy and core design, and current lead maintainer. The project is explicitly designed to evolve toward RFC-governed community maintenance as it matures — the `rfcs/` directory, with `drafts/` and `accepted/` subfolders and a standard proposal template, is already in place for this purpose.

**Community support.** Eye of Horus is community-funded rather than venture-funded at this stage. Contributions support research, documentation, compiler development, tooling, translation, infrastructure, testing, CI/CD, and website hosting. The full, current list of official donation channels (PayPal, bank transfer, and cryptocurrency) is maintained in the repository's `DONATE.md` and is intentionally not duplicated here, so that a single authoritative copy of payment details exists and stays current.

## 13. Security Posture

Consistent with the project's `SECURITY.md` and `SECURITY_MODEL.md`, this whitepaper states plainly: **Eye of Horus is pre-alpha research software and should not be used to process untrusted code in production.**

Explicitly out of scope today:

- The φ·π memory model is **not** a security feature (Section 6).
- The current CLI is **not** a sandbox.
- Safe execution of untrusted programs is **not yet defined.**

Planned security requirements, to be delivered alongside the Phase 1 parser and runtime (Section 11):

- Bounded parser input handling;
- Deterministic diagnostics (no undefined behavior on malformed `.eoh` files);
- Runtime resource limits;
- Safe file- and network-access defaults;
- Fuzz testing of the parser and runtime once both exist.

## 14. Conclusion

Eye of Horus is not a competitor to Python, Rust, or any language people use to ship production software, and this document has tried to say so plainly rather than let ambition blur into overclaiming. Its case for continued support rests on a narrower, and sturdier, foundation: a rigorously specified, honestly scoped, and genuinely novel combination of ideas — field-triggered activation, continuous 3D geometric instructions, and irrational-constant memory addressing — proposed and pioneered by a single named author, in a well-established tradition of esoteric languages that has produced real academic study, real community infrastructure, and real cultural impact for over fifty years. What exists today is small and verifiable; what is planned is ambitious and clearly labeled as such. Supporting this project means supporting a small, well-scoped piece of applied programming-language research, released back to the world under a permissive open-source license.

## 15. About the Founder

**Ciprian Stefan Plesca** is the founder of **AgentFlow Enterprise** (agentflow-enterprise.com), an AI-assisted revenue-operations platform focused on lead qualification, workflow visibility, and auditable automation for growth-stage teams. Eye of Horus is his independent research and open-source initiative — conceived, philosophically defined, and architecturally specified by him, with the reference Rust implementation actively in progress under his maintainership. He is the language's originating designer and pioneer.

## 16. How to Cite

Eye of Horus maintains a machine-readable citation file (`CITATION.cff`) at the repository root. The suggested citation for academic or technical reference is:

```
Plesca, C. S. (2026). Eye of Horus (Version 0.1.0-draft) [Computer software].
https://github.com/Ciprian-LocalPulse/Eye-of-Horus
```

## 17. References

1. Wikipedia contributors, "Esoteric programming language," *Wikipedia*. https://en.wikipedia.org/wiki/Esoteric_programming_language
2. "Esoteric programming language," *Esolang Wiki*. https://esolangs.org/wiki/Esoteric_programming_language
3. *Esolang: the esoteric programming languages wiki* (community portal and language index). https://esolangs.org/wiki/Main_Page
4. Mateas, M.; Montfort, N., "A Box, Darkly: Obfuscation, Weird Languages, and Code Aesthetics," Digital Arts and Culture (DAC 2005).
5. angrykoala, "awesome-esolangs: Curated list of awesome Esoteric languages and resources," GitHub. https://github.com/angrykoala/awesome-esolangs

---

*This document is released by its author as a public research whitepaper. Eye of Horus — its name, syntax, and specification as described herein — originates with Ciprian Stefan Plesca and is licensed under the Apache License 2.0 alongside its reference implementation.*
