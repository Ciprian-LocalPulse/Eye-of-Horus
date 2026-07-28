# Manifesto

## The Question We Are Asking

Eye of Horus exists to explore a simple question with serious discipline: **what if code were geometry first?** Not geometry as decoration on top of a conventional text-based language, and not geometry as a marketing metaphor, but geometry as the substrate in which values live, in which storage addresses are derived, and in which execution itself propagates. This is a research question. Research questions deserve honest experiments, not premature victory laps.

## What We Value

**Curiosity over hype.** We are pursuing this idea because it is interesting and under-explored, not because we believe it is destined to replace existing languages. A project motivated by curiosity can afford to report negative results; a project motivated by hype cannot, and that difference in incentive shapes everything else in this manifesto.

**Rigor over mystique.** The project's vocabulary borrows from physics and sacred geometry — "pulse," "field," "phi-pi addressing," "Higgs" — because those metaphors are genuinely useful for reasoning about expanding activation fronts and golden-ratio-based spatial quantization. They are not offered as evidence of deeper significance. Every metaphorical name in this codebase is paired, in its governing document, with a precise mathematical definition and an explicit statement of what it does *not* claim. See, for example, the non-claims statement in the white paper regarding the phi-pi addressing scheme.

**Public critique over private certainty.** A research project that only faces critique from within its own team accumulates blind spots. The RFC process (see [`RFC_PROCESS.md`](RFC_PROCESS.md)), the open issue tracker, and the explicit invitation for outside programming-language researchers to test, break, and challenge the model's claims are not procedural formalities; they are the mechanism by which this project is allowed to be wrong in public and to correct course.

## What We Will Not Do

**We will not claim production readiness before it exists.** Every document in this repository distinguishes, explicitly and by section heading where useful, between what is implemented and tested today and what is planned, speculative, or merely an open research question. A reader should never have to guess which category a given sentence falls into.

**We will not publish benchmark narratives without reproducible evidence.** A performance claim without source code, hardware specification, compiler version, and methodology is not a benchmark; it is an anecdote. [`PERFORMANCE.md`](PERFORMANCE.md) states this policy in engineering terms.

**We will not convert metaphors into scientific claims.** The golden ratio, the "Higgs" pulse, and the sacred-geometry-adjacent naming throughout this project are design choices with stated (and in some cases explicitly speculative) engineering rationale — see RFC 0001 for the fullest example of this discipline applied to a single design decision. They are never evidence of correctness, security, or computational power. A claim of that kind requires a proof, a benchmark, or a citation, not a name.

## What We Will Do

**We will build carefully.** The reference implementation grows in small, independently testable increments, each backed by tests, rather than as a large speculative dump of unverified functionality.

**We will document honestly.** Every planned feature is marked as planned. Every open research question — Turing-completeness, formal type soundness, whether the phi-pi quantum actually improves spatial locality — is stated as open, with a pointer to where it is tracked, rather than quietly assumed to be resolved in the project's favor.

**We will invite the programming-language community to test the idea in the open.** Eye of Horus is licensed under Apache-2.0, developed in a public repository, and governed by an RFC process specifically so that its central hypothesis — that geometry can serve as a useful semantic layer for a programming language, not merely a visualization layer bolted on afterward — can be examined, reproduced, and, if warranted, refuted by people outside the founding team.

## A Note on Ambition

None of the restraint described above is a hedge against ambition. The project's long-term goals, stated plainly in [`VISION.md`](VISION.md), are genuinely large: a formally specified language, a full reference toolchain, visual debugging as a research contribution to program comprehension, and educational material that makes an unconventional computation model accessible to newcomers. Discipline about what is already true is what makes ambition about what might become true credible. A project that exaggerates its present state has no reliable way to demonstrate progress toward its future state; a project that reports its present state precisely can point to that report again in a year and show, honestly, how far it has come.
