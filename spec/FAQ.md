# Frequently Asked Questions

## Is Eye of Horus production ready?

No. It is research-stage, pre-alpha software. See the status table in [`README.md`](README.md) and the phased plan in [`ROADMAP.md`](ROADMAP.md) for exactly what exists today versus what is planned.

## What exists today?

A professional, documented repository; a substantially complete draft of the formal specification for the language core and its operational semantics (see [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) and [`spec/SEMANTICS.md`](spec/SEMANTICS.md)); an RFC process with one accepted RFC (phi-pi addressing); illustrative `.eoh` examples; and a minimal Rust workspace scaffold (`eoh-core`, `eoh-cli`).

## What does not exist yet?

A complete parser covering the full grammar with production-quality diagnostics, a standalone compiler or bytecode VM, a runtime that executes the full instruction set end to end, a standard library, a package manager, a finished Language Server Protocol implementation, a finished VS Code extension, and a syntax that has been declared stable through the RFC process. See [`ARCHITECTURE.md`](ARCHITECTURE.md) for the precise boundary between existing and planned implementation.

## Is the phi-pi memory model secure?

No. The address function is deterministic and fully public; see [`MEMORY_MODEL.md`](MEMORY_MODEL.md) for its definition and [`SECURITY_MODEL.md`](SECURITY_MODEL.md) for the explicit statement that it is not, and was never intended to be, a security or obfuscation mechanism. It may be studied as a spatial-hashing and locality idea, and as a teaching object for cache and locality tradeoffs.

## Is Eye of Horus Turing-complete?

Not proven. This is stated as an open research question in the white paper, not assumed either way. Resolving it requires either a constructive reduction from a known Turing-complete model or a proof of a computational limitation, neither of which currently exists.

## Why Rust?

Rust offers memory safety without a garbage collector, a mature testing and tooling ecosystem (`cargo test`, `cargo fmt`, `clippy`), and a performance profile suitable for an interpreter and, eventually, a geometric runtime — see [`DESIGN_PRINCIPLES.md`](DESIGN_PRINCIPLES.md) Principle 5 for the fuller statement of what this choice does and does not guarantee about the interpreter's safety when running untrusted programs.

## Why does the project use physics and sacred-geometry-adjacent names like "Higgs pulse" and "phi-pi addressing"?

Because those names are genuinely descriptive of the underlying mechanism — an expanding activation front, and a golden-ratio-based spatial quantization — not because the project claims any deeper physical or esoteric significance. See [`MANIFESTO.md`](MANIFESTO.md) for the project's explicit commitment to pairing every such name with a precise mathematical definition and a statement of what it does not claim.

## Is this an esoteric programming language?

Eye of Horus shares lineage with the esoteric-language tradition — the white paper explicitly positions it relative to Befunge's two-dimensional instruction grid and Piet's image-based execution model — but its stated goals (a formal specification, a tested reference implementation, educational material, and a governed RFC process) are closer to those of a research language than a purely recreational one. Readers can judge for themselves once the language core is complete enough to try.

## How can I contribute?

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for useful contribution areas and the development setup, [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) for community standards, and [`GOVERNANCE.md`](GOVERNANCE.md) for how decisions are currently made.

## How can I support the project financially?

See [`DONATE.md`](DONATE.md) for the currently published donation methods and what contributions fund.
