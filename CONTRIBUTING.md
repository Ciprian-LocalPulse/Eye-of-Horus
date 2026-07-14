# Contributing to Eye of Horus

Thank you for considering a contribution. Eye of Horus is early research, so careful critique is as valuable as code.

## Useful Contributions

- clarify documentation;
- improve examples while marking speculative behavior;
- add Rust tests for implemented functions;
- propose language changes through RFCs;
- identify ambiguity in the specification;
- improve CI, formatting, or release automation.

## Development Setup

```bash
cargo test
cargo fmt --all
cargo clippy --workspace --all-targets
```

## Contribution Rules

- Do not present planned features as implemented.
- Do not add benchmark claims without reproducible benchmark code and methodology.
- Do not describe the phi-pi addressing scheme as a security mechanism.
- Use RFCs for syntax, semantics, memory model, runtime, or standard-library changes.

By contributing, you agree that your contribution is licensed under Apache-2.0.
