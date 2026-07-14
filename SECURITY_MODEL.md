# Security Model

Status: draft.

Eye of Horus currently has no hardened execution model.

## Non-Goals

- The phi-pi memory model is not a security feature.
- The current CLI is not a sandbox.
- The repository does not yet define safe execution of untrusted programs.

## Planned Requirements

- bounded parser input;
- deterministic diagnostics;
- runtime resource limits;
- safe file and network defaults;
- fuzzing for parser and runtime once implemented.
