# Governance

## Current Structure

Eye of Horus begins as a founder-led research project, originated and maintained by Ciprian Stefan Plesca. This is stated plainly rather than obscured, because founder-led governance carries different tradeoffs than community governance, and contributors deserve to know which one they are working within: decisions can move quickly, but they currently rest with a small number of people rather than a broad, formally constituted body. The explicit governance goal is to move toward transparent, broader community participation as the project matures, in step with the growth described in [`ROADMAP.md`](ROADMAP.md).

## Maintainer Responsibilities

Maintainers are responsible for:

- **protecting project honesty and research integrity** — enforcing the distinction between implemented, planned, and speculative work described in [Design Principle 1](DESIGN_PRINCIPLES.md#1-honesty-before-spectacle) across all project documentation, not only in code;
- **reviewing contributions** against the standards in [`CONTRIBUTING.md`](CONTRIBUTING.md) and the [design principles](DESIGN_PRINCIPLES.md);
- **enforcing the code of conduct** described in [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md), including triaging reports sent to the project's contact address;
- **keeping implemented, planned, and speculative work clearly labeled** in the roadmap, changelog, and specification documents;
- **stewarding the RFC process** described in [`RFC_PROCESS.md`](RFC_PROCESS.md), including deciding when a proposed change requires an RFC versus a direct pull request.

## Decision Process

- **Minor documentation and tooling changes** — typos, clarifications, formatting, CI configuration, and similar low-risk changes — may be accepted through ordinary pull-request review without a formal RFC.
- **Language semantics, syntax, memory model, runtime behavior, and standard-library design** require RFC discussion before implementation, per [`RFC_PROCESS.md`](RFC_PROCESS.md). This is not a bureaucratic formality; it is the mechanism that keeps the language's evolution auditable, consistent with [Design Principle 6](DESIGN_PRINCIPLES.md#6-community-design-through-rfcs).
- **Disputed decisions** that cannot be resolved through ordinary discussion are decided, at this stage of the project, by the founding maintainer, with the expectation that the reasoning behind the decision is recorded in the relevant RFC or issue thread rather than left implicit.

## Future Structure

As the project and its contributor base grow, future working groups may be established to cover distinct areas of responsibility, including:

- **Compiler and runtime** — the interpreter, parser, and execution pipeline described in [`ARCHITECTURE.md`](ARCHITECTURE.md);
- **Specification** — the formal grammar and semantics documents in [`spec/`](spec/README.md);
- **Tooling** — editor integrations, the language server, and the CLI;
- **Documentation** — the language book, website, and this documentation set itself;
- **Security** — stewarding [`SECURITY_MODEL.md`](SECURITY_MODEL.md) toward the planned requirements it lists;
- **Education** — the tutorial book and any future course-style material referenced in [`VISION.md`](VISION.md).

The formation of these groups, their scope, and their relationship to the founder-led decision process above will itself be proposed and recorded through the RFC process once the project reaches a stage where they are warranted.
