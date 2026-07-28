# Eye of Horus White Paper

**Status:** Version 0.1, draft.

This directory contains the public research white paper for Eye of Horus and its supporting materials. The white paper introduces the project's motivating question, positions it against related work in non-linear and spatial programming languages, gives the formal memory and execution models, and states the project's open research problems explicitly. It is a research document, not a claim that the language is complete — see its own §1.3 ("What this paper is not claiming") for the fullest statement of that boundary.

## Contents

- [`eye-of-horus-whitepaper.md`](eye-of-horus-whitepaper.md) — the primary research-paper-style treatment, with related work, a worked semantics example, a reproducibility section, and a glossary.
- [`Eye_of_Horus_Whitepaper.md`](Eye_of_Horus_Whitepaper.md) — a companion, more narrative presentation of the same material, including implementation-status verification and citation guidance.
- [`Figures.md`](Figures.md) — the index of figures referenced by or planned for the white paper.
- [`Future_Research.md`](Future_Research.md) — open research directions raised but not resolved by the current draft.
- [`References.md`](References.md) — the bibliography and prior-work citations underlying the white paper's positioning section.

## How to Cite

See the "How to Cite" section of [`Eye_of_Horus_Whitepaper.md`](Eye_of_Horus_Whitepaper.md) for the currently recommended citation form for this draft.

## Relationship to the Formal Specification

The white paper argues the research case for Eye of Horus's design and states open problems at a level appropriate for readers unfamiliar with the project. The normative, versioned rules that an implementation must actually satisfy live in [`spec/`](../spec/README.md), not here; where the white paper and the specification appear to disagree on a point of implemented behavior, the specification governs.
