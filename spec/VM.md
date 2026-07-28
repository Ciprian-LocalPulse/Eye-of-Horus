# Virtual Machine

## Status

Research idea; not implemented as a standalone crate. This document exists to give the idea a proper hearing, since the bytecode instruction set that a future VM would dispatch is already fully specified in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §6, ahead of any decision about whether a dedicated VM crate is warranted.

## The Question

A virtual machine — an intermediate bytecode representation with its own dispatch loop, separate from a direct AST-walking interpreter — is not needed to give Eye of Horus a working execution model. `eoh-runtime` (see [`ARCHITECTURE.md`](ARCHITECTURE.md)) can, in principle, walk the AST and drive pulse simulation directly. A VM becomes useful specifically when at least one of the following becomes true:

- **Portability** — a stable bytecode format lets independent tools (a future visualizer, alternative host runtimes, or foreign-language embeddings) consume Eye of Horus programs without depending on the AST's internal Rust representation.
- **Visualization** — a bytecode stream with explicit instruction boundaries may be easier to step through and render in a debugger than an AST walk, since each instruction corresponds to one observable machine step (see the small-step judgments in [`spec/SEMANTICS.md`](spec/SEMANTICS.md) §4, which are already written at bytecode granularity for exactly this reason).
- **Testing** — a fixed instruction set gives a stable target for golden-output tests that do not need to change every time the parser's internal AST representation is refactored.
- **Optimization** — a separate IR stage creates a natural place to apply optimizations (constant folding, dead-pulse elimination) without complicating the parser or the AST-to-execution boundary.

## What Is Already Specified

Because [Design Principle 2](DESIGN_PRINCIPLES.md#2-specification-before-stabilization) calls for specification before stabilization, the instruction set a VM would need is already defined: `PushFloat`, `PushBool`, `PushStr`, `Load`, `Store`, the arithmetic instructions, `DeclareVertex`, `DeclareShape`, `EmitPulse`, `Call`, `Return`, `Jump`, `JumpIf`, and `Halt`, along with the `BytecodeImage` envelope (instructions, interned strings, source path, and a version field used to reject unrecognized schema versions rather than attempt best-effort execution). See [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §6–7 for the complete, authoritative table.

## What Is Not Yet Decided

Whether this instruction set is dispatched by a genuinely separate `eoh-vm` crate, or folded directly into `eoh-runtime` as an internal implementation detail with no separately versioned public format, is an open architectural question. Arguments for separation (clean portability boundary, independent testability) and arguments against (avoiding a crate boundary and serialization format for a research-stage language with no external consumers yet) both have merit and neither has been settled. Future RFCs should justify whichever choice is made, explicitly weighing the tradeoffs above rather than defaulting to either option by convention.

## Relationship to Other Documents

See [`COMPILER.md`](COMPILER.md) for the closely related question of whether and when a compiler (as opposed to a directly interpreted or bytecode-executed AST) is warranted, and [`RUNTIME.md`](RUNTIME.md) for how pulse scheduling and activation are expected to be driven regardless of which execution strategy is chosen.
