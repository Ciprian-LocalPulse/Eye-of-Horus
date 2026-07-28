# Security Model

## Status

Draft. Eye of Horus currently has no hardened execution model, and this document exists specifically to prevent the project's evocative naming and geometric framing from being mistaken for security properties that have not been established.

## Non-Goals — Stated Explicitly

These statements are deliberately unambiguous, because they correct a class of misunderstanding this project has anticipated:

- **The phi-pi memory model is not a security feature.** It is a deterministic, public spatial-hashing function for storage locality, described fully in [`MEMORY_MODEL.md`](MEMORY_MODEL.md). Its formula is published, its inputs are ordinary program-declared coordinates, and knowing the formula gives no information advantage to an attacker because the formula was never intended to hide anything.
- **The current CLI is not a sandbox.** Running `eoh-cli` against a `.eoh` file today provides no isolation guarantees against a maliciously crafted input beyond whatever the host operating system's ordinary process boundaries provide.
- **The repository does not yet define safe execution of untrusted programs.** There is no resource-limiting, no capability model, and no formally reasoned isolation boundary for running a `.eoh` program of unknown provenance.

Anyone building on Eye of Horus today should treat every `.eoh` program as running with the full privileges of the process executing it, exactly as they would for an untrusted script in any other general-purpose language with no sandbox.

## Planned Requirements

Before Eye of Horus can respons­ibly claim any degree of safety for untrusted input, the following must exist and be tested, not merely designed:

- **Bounded parser input** — explicit limits on source file size, nesting depth, and token count, so that a maliciously crafted input cannot exhaust memory or stack space during parsing alone.
- **Deterministic diagnostics** — parser and static-check errors that do not depend on incidental factors such as hash-map iteration order, both for reproducibility and because non-deterministic error reporting is itself a poor foundation for any later security reasoning.
- **Runtime resource limits** — bounds on live pulse count, spatial-field size, and maximum tick count, as already flagged as an open runtime question in [`RUNTIME.md`](RUNTIME.md).
- **Safe file and network defaults** — since no such I/O capability exists in the language yet, this requirement is prospective: if and when the standard library (see [`STANDARD_LIBRARY.md`](STANDARD_LIBRARY.md)) introduces file or network operations, they must default to denied rather than permitted, with any capability grant explicit and auditable.
- **Fuzzing for parser and runtime** once each exists in a form stable enough to fuzz productively, integrated into continuous integration rather than run as a one-time exercise.

## Reporting

Security issues in the current scaffold — for example, a panic or resource exhaustion triggerable by a crafted input to the existing `eoh-core` or `eoh-cli` code — should be reported following the process in [`SECURITY.md`](SECURITY.md), even though the project does not yet claim a hardened boundary; early reports of this kind directly inform which planned requirements above are prioritized.

## Relationship to Other Documents

This document states policy and intent. [`SECURITY.md`](SECURITY.md) states the operational vulnerability-reporting process. [`MEMORY_MODEL.md`](MEMORY_MODEL.md) contains the fuller argument for why phi-pi addressing specifically is not, and was never intended to be, a security mechanism.
