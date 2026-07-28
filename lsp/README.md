# Language Server

This directory is a placeholder structure for a future Eye of Horus Language Server Protocol (LSP) implementation. No server is implemented yet; see [`design.md`](design.md) for the planned capabilities and [`ROADMAP.md`](../ROADMAP.md) Phase 2 for its tracked timeline relative to the rest of the toolchain.

## Why an LSP Matters for This Project

An LSP implementation is planned to depend directly on the same parser and static-check logic used by the reference interpreter (see [`ARCHITECTURE.md`](../ARCHITECTURE.md)), rather than maintaining a separate, parallel understanding of the language. This dependency ordering means meaningful LSP work cannot begin in earnest until the parser and static checks described in [`ROADMAP.md`](../ROADMAP.md) Phase 1 are substantially complete — an LSP built against an unstable or partial grammar would need to be rewritten as that grammar changes.

## Status

Placeholder. Design notes exist in [`design.md`](design.md); no server code exists yet.
