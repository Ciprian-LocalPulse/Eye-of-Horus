# Roadmap

This roadmap separates implemented work from planned work, and deliberately omits calendar dates until maintainers can commit to a schedule responsibly, consistent with [Design Principle 1](DESIGN_PRINCIPLES.md#1-honesty-before-spectacle). Phases are ordered by dependency, not by fixed duration; a phase begins in earnest once its prerequisites from the previous phase are met, and phases may overlap where their work streams are independent.

## Phase 0: Public Research Repository

**Status: in progress.**

- repository structure, licensing, and contribution scaffolding;
- Apache-2.0 license (see [`LICENSE`](LICENSE));
- contribution and governance documents ([`CONTRIBUTING.md`](CONTRIBUTING.md), [`GOVERNANCE.md`](GOVERNANCE.md), [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md));
- white paper draft ([`whitepaper/`](whitepaper/README.md));
- formal specification skeleton and, for the language core and semantics, a substantially complete first draft ([`spec/`](spec/README.md));
- minimal Rust workspace scaffold (`eoh-core`, `eoh-cli`).

## Phase 1: Language Core

**Status: planned.**

- lexer and parser with full diagnostic coverage, implementing the grammar in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §3;
- AST construction and static checks (name resolution, shape arity, coordinate validity);
- spatial matrix (field) representation, backed by the phi-pi addressing scheme in [`MEMORY_MODEL.md`](MEMORY_MODEL.md);
- deterministic pulse simulation implementing [`spec/SEMANTICS.md`](spec/SEMANTICS.md), including resolution of the currently documented semantic gaps (strict unbound-name faulting for `Load`/`Store`, and a decoupled tick model);
- execution semantics for `IF`/`ELSE` and `LOOP`/`BREAK`/`CONTINUE`, currently parsed but not yet load-bearing per [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §8;
- clear value encoding rules, closing the open questions in [`TYPE_SYSTEM.md`](TYPE_SYSTEM.md) sufficiently for a first stable core;
- first fully executable `.eoh` example programs, verified in continuous integration;
- a test suite for the parser and core semantics comprehensive enough to satisfy [Design Principle 3](DESIGN_PRINCIPLES.md#3-tests-before-claims).

## Phase 2: Tooling Alpha

**Status: planned.**

- a CLI interpreter prototype capable of running arbitrary `.eoh` programs end to end, not only reporting status;
- a visualizer prototype rendering spatial state and pulse activation fronts, as described in [`VISION.md`](VISION.md);
- VS Code syntax highlighting (see [`editors/vscode/README.md`](editors/vscode/README.md));
- initial Language Server Protocol diagnostics (see [`lsp/design.md`](lsp/design.md));
- RFC-backed stabilization of the syntax areas listed as open in [`SYNTAX.md`](SYNTAX.md), including Unicode identifiers and multi-file `IMPORT` resolution.

## Phase 3: Research and Community

**Status: planned.**

- formal semantics review, including community scrutiny of the small-step judgments in [`spec/SEMANTICS.md`](spec/SEMANTICS.md);
- computational-power analysis, addressing the open Turing-completeness question stated in the white paper;
- educational materials, completing the chapters planned in the [language book](book/src/SUMMARY.md);
- standard-library experiments, per [`STANDARD_LIBRARY.md`](STANDARD_LIBRARY.md), each backed by an RFC and a test suite;
- formation of community working groups, per the future-structure section of [`GOVERNANCE.md`](GOVERNANCE.md).

## Phase 4: Empirical Evaluation

**Status: planned.**

- reproducible benchmarking of the phi-pi addressing scheme against simpler alternatives, satisfying the methodology bar in [`PERFORMANCE.md`](PERFORMANCE.md);
- controlled evaluation of whether spatial/visual debugging measurably improves program comprehension relative to conventional debuggers, as proposed as a research goal in [`VISION.md`](VISION.md);
- publication of results regardless of whether they favor the project's original design hypotheses, per the manifesto's commitment to reporting negative results.

## Tracking Changes to This Roadmap

Movement between phases, and any reordering of the items within them, is recorded in [`CHANGELOG.md`](CHANGELOG.md) so that the roadmap's history — not only its current snapshot — remains auditable.
