# Contributing to Eye of Horus

Thank you for considering a contribution. Eye of Horus is early-stage research, and at this stage careful critique, precise documentation, and rigorous tests are exactly as valuable as new code — often more so, since a wrong claim caught in review costs far less than one shipped and later retracted.

## Useful Contributions

- **Clarify documentation.** If a document leaves you unsure whether something is implemented, planned, or speculative, that ambiguity is a bug in the documentation, per [Design Principle 1](DESIGN_PRINCIPLES.md#1-honesty-before-spectacle). Fixing it is a genuine contribution.
- **Improve examples while marking speculative behavior.** New `.eoh` examples are welcome; each should state clearly whether it is verified to run against the current reference implementation or is illustrative of planned syntax, following the convention established in [`README.md`](README.md) and the [language book](book/00-introduction.md).
- **Add Rust tests for implemented functions.** Every behavior claimed anywhere in this repository's documentation should be exercised by an automated test; gaps between the two are worth closing.
- **Propose language changes through RFCs.** Syntax, semantics, memory model, runtime, and standard-library changes go through the process in [`RFC_PROCESS.md`](RFC_PROCESS.md), not through direct pull requests.
- **Identify ambiguity in the specification.** [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) and [`spec/SEMANTICS.md`](spec/SEMANTICS.md) invite exactly this kind of scrutiny; an issue that says "this rule doesn't cover case X" is more useful at this stage than a large unsolicited implementation of case X.
- **Improve CI, formatting, or release automation.** Tooling contributions that make the honesty and testing requirements above easier to enforce automatically are always welcome.

## Development Setup

```bash
cargo test
cargo fmt --all
cargo clippy --workspace --all-targets
```

A contribution should pass all three before being opened as a pull request. `cargo test` failures block review; `cargo fmt` and `cargo clippy` findings should be resolved or, if a `clippy` lint is being deliberately overridden, justified in a code comment.

## Contribution Rules

- **Do not present planned features as implemented**, in code comments, documentation, examples, or commit messages.
- **Do not add benchmark claims without reproducible benchmark code and methodology**, per [`PERFORMANCE.md`](PERFORMANCE.md).
- **Do not describe the phi-pi addressing scheme as a security mechanism**, per [`MEMORY_MODEL.md`](MEMORY_MODEL.md) and [`SECURITY_MODEL.md`](SECURITY_MODEL.md).
- **Use RFCs for syntax, semantics, memory model, runtime, or standard-library changes**, per [`RFC_PROCESS.md`](RFC_PROCESS.md); documentation clarifications and test additions do not require an RFC.

## Review Expectations

Reviewers apply the [design principles](DESIGN_PRINCIPLES.md) as the primary review standard: a technically correct change that blurs the line between implemented and planned behavior will be asked to clarify that line before merging, regardless of the underlying code quality. This is not a judgment on the contributor's work; it is the project's stated editorial policy applied consistently.

## Licensing

By contributing, you agree that your contribution is licensed under Apache-2.0, the license under which the entire project is distributed (see [`LICENSE`](LICENSE)).

## Getting Help

If you are unsure whether a contribution needs an RFC, whether it fits the project's current roadmap phase, or simply where to start, open a GitHub Discussion or issue — see [`SUPPORT.md`](SUPPORT.md) for the full set of community channels.
