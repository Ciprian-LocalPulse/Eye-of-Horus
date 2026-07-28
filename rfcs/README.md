# RFCs

Requests for Comments (RFCs) are the required process for any change to Eye of Horus's syntax, semantics, memory model, runtime scheduling, standard-library design, or compatibility policy. See [`RFC_PROCESS.md`](../RFC_PROCESS.md) for the full process, quality bar, and the categories of change that require an RFC.

## Directory Structure

- [`0000-template.md`](0000-template.md) — the required starting point for any new RFC; copy this file rather than writing an RFC from scratch.
- [`drafts/`](drafts/README.md) — in-progress RFC drafts, open for discussion.
- [`accepted/`](accepted/README.md) — RFCs that have been accepted and may proceed to implementation.
- [`0001-phi-pi-addressing.md`](0001-phi-pi-addressing.md) — the project's first accepted RFC, adopting the phi-pi addressing scheme described in [`MEMORY_MODEL.md`](../MEMORY_MODEL.md). It is kept at the top level of this directory as the reference example of the depth and honesty expected of every subsequent RFC, including its explicit treatment of an unproven design hypothesis.

## Getting Started

1. Read [`RFC_PROCESS.md`](../RFC_PROCESS.md) in full.
2. Copy [`0000-template.md`](0000-template.md) to `drafts/NNNN-your-title.md`, using the next available number.
3. Write the motivation, design, drawbacks, and alternatives sections completely — an RFC that omits drawbacks or alternatives will be returned for revision.
4. Open a pull request referencing the draft.
