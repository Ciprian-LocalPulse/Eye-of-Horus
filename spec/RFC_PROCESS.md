# RFC Process

## Purpose

The RFC (Request for Comments) process exists so that significant design decisions in Eye of Horus are proposed, argued, and recorded in a durable, citable form, rather than made implicitly through code changes. A future contributor should be able to answer "why does the language work this way?" by reading an RFC, in the same way that [RFC 0001](rfcs/0001-phi-pi-addressing.md) documents the full reasoning, tradeoffs, and open questions behind the phi-pi addressing scheme.

## What Requires an RFC

- **Syntax changes** — any addition, removal, or modification of the grammar specified in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §3.
- **Semantic rules** — any change to the operational semantics in [`spec/SEMANTICS.md`](spec/SEMANTICS.md), including resolving any of the currently documented open gaps.
- **Memory model changes** — any modification to the phi-pi addressing scheme or an alternative proposal, per [`MEMORY_MODEL.md`](MEMORY_MODEL.md).
- **Runtime scheduling** — changes to pulse propagation, activation ordering, or the tick model described in [`RUNTIME.md`](RUNTIME.md).
- **Standard-library design** — any new module or public interface, per [`STANDARD_LIBRARY.md`](STANDARD_LIBRARY.md).
- **Compatibility policy** — changes to the versioning guarantees described in [`VERSIONING.md`](VERSIONING.md).

Documentation clarifications, test additions, and tooling improvements that do not change the language's observable behavior do not require an RFC; see [`GOVERNANCE.md`](GOVERNANCE.md) for the boundary between RFC-required and directly-reviewable changes.

## Process

1. **Copy the template.** Start from [`rfcs/0000-template.md`](rfcs/0000-template.md) rather than writing an RFC from a blank file, so that every proposal covers the same required sections.
2. **Choose a descriptive title** and file the draft under [`rfcs/drafts/`](rfcs/drafts/README.md) with the next available number.
3. **Write the motivation, design, drawbacks, and alternatives sections in full.** An RFC that skips drawbacks or alternatives is treated as incomplete; RFC 0001 is the reference example of what a thorough treatment of these sections looks like, including its explicit acknowledgment of an unproven design hypothesis.
4. **Open a pull request** referencing the draft RFC.
5. **Discuss until the RFC reaches one of four outcomes:** *accepted* (moved to [`rfcs/accepted/`](rfcs/accepted/README.md) and implementation may begin), *revised* (returned to the author with requested changes), *postponed* (recorded as worth revisiting once a stated precondition is met), or *rejected* (closed with the reasoning recorded in the pull request, for future reference).

## Quality Bar for Acceptance

An RFC is ready for acceptance when it satisfies the same bar [Design Principle 1](DESIGN_PRINCIPLES.md#1-honesty-before-spectacle) applies to all project documentation: it must state plainly what is being claimed, what is speculative, and what remains an open question after the change is implemented. An RFC that resolves its own "Unresolved Questions" section by simply deleting it, rather than answering the questions, has not met this bar.

## Relationship to Governance

The RFC process is the primary mechanism through which [`GOVERNANCE.md`](GOVERNANCE.md) delegates design authority away from any single maintainer's unilateral judgment, even during the project's current founder-led phase.
