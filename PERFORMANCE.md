# Performance

## Status

No benchmarks have been published. This is a policy document, not a results document: it defines what would have to be true before Eye of Horus publishes a performance claim, so that the absence of numbers today is a deliberate consequence of that policy rather than an oversight.

## Why No Numbers Exist Yet

A performance claim without reproducible evidence is not a benchmark; it is an anecdote, and anecdotes are explicitly excluded by [Design Principle 1](DESIGN_PRINCIPLES.md#1-honesty-before-spectacle) and the manifesto's commitment against publishing "benchmark narratives without reproducible evidence." Because the language core, runtime, and memory model are all still under active design (see [`ARCHITECTURE.md`](ARCHITECTURE.md), [`RUNTIME.md`](RUNTIME.md), and [`MEMORY_MODEL.md`](MEMORY_MODEL.md)), any benchmark run today would measure a moving target and could not be meaningfully reproduced against a later version of the same claim.

## Requirements for a Future Benchmark

Any performance claim published by this project — in documentation, in the white paper, in release notes, or in third-party-facing material — must be accompanied by:

- **reproducible source code** for both the benchmark harness and the exact program being measured, committed to the repository or linked from it;
- **hardware and operating system details**, including CPU model, memory, and OS version;
- **compiler version**, including the exact Rust toolchain version (`rustc --version`) and relevant build flags;
- **methodology**, including how many runs were taken, how variance was handled, and what was held constant across comparisons;
- **comparison targets and rationale**, if the benchmark compares against another language or implementation, stating explicitly why that comparison target was chosen and what it does and does not establish;
- **limitations**, stated in the same document as the results, not deferred to a separate caveats page.

A benchmark that satisfies all of the above but shows an unfavorable result for Eye of Horus must still be published if it is otherwise relevant to a documented open question — for example, the phi-pi locality hypothesis in [`MEMORY_MODEL.md`](MEMORY_MODEL.md) is exactly the kind of question this project needs an honest answer to, whichever direction the answer points.

## What Can Be Discussed Today

Until reproducible benchmarks exist, performance discussion in this repository is limited to architectural planning: which design choices are *expected*, on ordinary algorithmic grounds, to matter for performance (for example, the choice of a stack-based bytecode representation in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §6, or the O(1) lattice-address lookup versus a hypothetical spatial-index alternative discussed in RFC 0001). Such discussion should be phrased as expectation and hypothesis, not as a demonstrated result.

## Tracking

Benchmark methodology and initial results are tracked as a Phase 4 item in [`ROADMAP.md`](ROADMAP.md), deliberately placed after the language core, runtime, and standard library reach enough stability that a benchmark's target does not shift out from under it before publication.
