# Eye of Horus Book

This directory contains the long-form tutorial and reference book for Eye of Horus, aimed at readers who want to *use* the language and its tooling, complementing the theoretical treatment in the [white paper](../whitepaper/README.md) and the normative rules in [`spec/`](../spec/README.md).

## Structure

The book is organized as two parallel tracks that are in the process of being merged into a single coherent structure:

- **`00-introduction.md`** — a complete, tested tutorial chapter (Chapter 0), verified against the current reference implementation and continuous integration. This is the best current starting point for a new reader.
- **`src/`** — an [mdBook](https://rust-lang.github.io/mdBook/)-compatible source tree (`SUMMARY.md` and per-chapter files), intended to become the book's build structure as more chapters are written. See [`src/SUMMARY.md`](src/SUMMARY.md) for the current table of contents and [`src/future-chapters.md`](src/future-chapters.md) for the planned chapter breakdown.

## Status

Chapter 0 is written and verified. Chapters 1 through 4 — vertices and shapes, the spatial field and phi-pi addressing, pulses and activation, and functions and arithmetic — are planned, per the outline in `00-introduction.md`'s "What's next" section, and tracked against [`ROADMAP.md`](../ROADMAP.md) Phase 3. Contributions toward these chapters are welcome; see [`CONTRIBUTING.md`](../CONTRIBUTING.md).

## Editorial Standard

Every code example in this book must either be verified to run against the current reference implementation (and exercised by continuous integration) or be clearly marked as illustrative of planned syntax, following the convention already established in Chapter 0. This is a direct application of [Design Principle 3](../DESIGN_PRINCIPLES.md#3-tests-before-claims) to educational material specifically.
