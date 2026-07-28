# Style Guide

## Status

Initial, applied consistently across this documentation set. This guide exists to make the editorial standards implicit in [`DESIGN_PRINCIPLES.md`](DESIGN_PRINCIPLES.md) checkable at the level of individual sentences and code changes.

## Documentation

- **Use precise language.** Prefer a specific, checkable statement ("the parser does not yet execute `LOOP` bodies") over a vague one ("looping is still being worked on").
- **Mark TODOs and open questions explicitly**, using a dedicated heading or sentence rather than leaving a gap the reader must infer.
- **Separate implemented features from planned features** at the point where a reader would naturally ask which category applies — usually the first mention of the feature in a given document, not buried in a later caveats section.
- **Avoid hype and unverifiable claims.** A superlative ("blazing fast," "revolutionary") is not evidence; a citation, a test, or a benchmark methodology is. See [`PERFORMANCE.md`](PERFORMANCE.md) for the concrete application of this rule to performance claims specifically.
- **Cross-reference rather than duplicate.** When two documents cover related ground — for example, this repository's root-level `SEMANTICS.md` and the fully formal `spec/SEMANTICS.md` — the less formal document should summarize and link to the authoritative one rather than maintain a separate, potentially drifting copy of the same content.
- **State what a name does not claim, immediately after introducing it, for any evocative or metaphorical name** (`Higgs pulse`, `phi-pi addressing`). This mirrors the manifesto's commitment to pairing metaphor with precise definition.

## Rust

- Run `cargo fmt --all` before submitting any change.
- Run `cargo clippy --workspace --all-targets` and resolve or explicitly justify every finding.
- Prefer small modules with tests over large modules that bundle multiple responsibilities, consistent with [Design Principle 7](DESIGN_PRINCIPLES.md#7-small-independently-testable-units).
- Do not introduce a new dependency without a clear, stated purpose in the pull request description; prefer the standard library or an existing workspace dependency where reasonably possible.
- Every public function that implements a construct described in the specification should have a doc comment linking to the relevant specification section.

## Eye of Horus Examples (`.eoh` files)

- Use the `.eoh` file extension for all example source files.
- Add comments explaining any behavior that is illustrative of planned syntax rather than a description of currently executable behavior, following the convention established in [`README.md`](README.md).
- Do not imply that a placeholder or illustrative example is executable until the parser and runtime actually support it end to end; where an example has been verified to run against the current implementation (for instance, by continuous integration), say so explicitly, as in the [language book's introduction](book/00-introduction.md).

## Applying This Guide to Pull Requests

Reviewers may request changes to bring a contribution into line with this guide even when the underlying technical content is correct, since documentation precision is treated as a first-class review criterion alongside correctness and test coverage, per [Design Principle 8](DESIGN_PRINCIPLES.md#8-documentation-is-part-of-the-deliverable-not-an-afterthought).
