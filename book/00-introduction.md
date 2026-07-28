# The Eye of Horus Book — Chapter 0: Introduction

Welcome. This is a tutorial-style introduction to Eye of Horus, written for
readers who want to *use* the language, not just read about its theory (for
that, see [`whitepaper/eye-of-horus-whitepaper.md`](../whitepaper/eye-of-horus-whitepaper.md)).

## Who this book is for

You should be comfortable with basic programming concepts (variables,
functions, types) in any language. No prior geometry or physics background
is assumed — we'll build up the spatial concepts from scratch.

## What makes Eye of Horus different, in plain terms

In most languages, you write:

```python
x = 5
y = x + 1
```

and `x` lives "somewhere" in memory you don't think about. In Eye of Horus,
every value's "somewhere" is an actual point in 3-D space, and that point
matters:

```eoh
VERTEX A 1.0, 0.0, 0.0
LET x = 5.0
```

Here, `A` is a named point at coordinates `(1, 0, 0)`. Internally, `x` (and
any value associated with a vertex) gets stored at a lattice address derived
from that point — see Chapter 2 for details. You don't normally need to
think about this to write simple programs, but it becomes important once
you start using `PULSE_HIGGS` to drive execution (Chapter 3).

## Installing the toolchain

```bash
git clone https://github.com/Ciprian-LocalPulse/Eye-of-Horus.git
cd Eye-of-Horus
cargo build --workspace --release
```

The `eoh` binary will be at `target/release/eoh`. Add it to your `PATH`, or
just run everything via `cargo run -p eoh-cli --`.

## Your first program

Create `hello.eoh`:

```eoh
ORIGIN 0.0, 0.0, 0.0
VERTEX HELLO 0.0, 0.0, 0.0
PULSE_HIGGS HELLO, v=1.0
```

Run it:

```bash
eoh run hello.eoh
```

You should see:

```
✓  executed in N ticks, 1 field cells occupied, 1 active pulses
```

That's it — you've declared a coordinate origin, a single vertex sitting
exactly on it, and emitted a pulse from that vertex. Because the pulse
originates exactly at `HELLO`'s position, it activates immediately.

## What's next

- **Chapter 1** (`01-vertices-and-shapes.md`) — declaring vertices, edges,
  and the built-in shape kinds (tetrahedron, cube, icosahedron, sphere).
- **Chapter 2** (`02-the-spatial-field.md`) — how the phi-pi addressing
  lattice works, and what it means for two vertices to "share" storage.
- **Chapter 3** (`03-pulses-and-activation.md`) — the Higgs-pulse model in
  depth, with worked timing diagrams.
- **Chapter 4** (`04-functions-and-arithmetic.md`) — `FN`, `LET`, and the
  expression grammar.

*(Chapters 1–4 are planned; contributions welcome — see `CONTRIBUTING.md`.
This introduction and the runnable examples in `examples/` are complete and
tested today.)*

## A note on the language's maturity

Eye of Horus is pre-alpha. The examples in this book are all verified to
run against the current reference implementation (`cargo test --workspace`
passes, and every `.eoh` file in `examples/` is exercised by CI). But
control flow (`IF`, `LOOP`) is not yet wired into execution — see
`ROADMAP.md` Phase 1. This book will grow alongside the implementation
rather than describing aspirational features as if they already worked.
