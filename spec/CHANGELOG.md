# Changelog

All notable changes to Eye of Horus are documented in this file. The format follows [Keep a Changelog](https://keepachangelog.com/), and the project intends to adopt [Semantic Versioning](https://semver.org/) once its first tagged release is cut, per the policy in [`VERSIONING.md`](VERSIONING.md).

Because Eye of Horus has not yet cut a `0.1.0` tag, every change to date is recorded under `[Unreleased]`. Entries are grouped using the standard Keep a Changelog categories (`Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security`) so that, once a first release is tagged, this file can be split into dated sections without restructuring its conventions.

## [Unreleased]

### Added

- Initial open-source repository structure, licensed under Apache-2.0.
- Research-stage governance, contribution, and community documents (`GOVERNANCE.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `RFC_PROCESS.md`).
- A formal specification set in `spec/`, including a substantially complete lexical grammar, EBNF grammar, static semantics, and a small-step operational semantics for the executable core, alongside a shorter bridging documentation layer at the repository root that indexes and contextualizes those normative documents.
- RFC 0001, documenting and accepting the phi-pi addressing scheme for the spatial field.
- A white paper draft (two companion documents: a narrative introduction and a research-paper-style treatment with related work, open problems, and a reproducibility section).
- A minimal Rust workspace scaffold (`eoh-core`, `eoh-cli`) with tests for core geometry utilities and the phi-pi address function.
- Illustrative `.eoh` example syntax, clearly distinguished from verified-executable examples per the project's documentation conventions.
- RFC, website, book, VS Code extension, and Language Server placeholder structures, each documenting their planned scope ahead of implementation.

### Changed

- N/A — no prior release exists to change from.

### Deprecated

- N/A.

### Removed

- N/A.

### Fixed

- N/A.

### Security

- No security-relevant changes have been made against a hardened boundary, because no such boundary yet exists; see [`SECURITY_MODEL.md`](SECURITY_MODEL.md).

## Maintaining This File

Every pull request that changes user-observable behavior, specification content, or governance process should include a corresponding entry under `[Unreleased]` in the appropriate category. Movement between [`ROADMAP.md`](ROADMAP.md) phases should also be reflected here, so that this file, rather than only the roadmap's current snapshot, preserves the project's history.
