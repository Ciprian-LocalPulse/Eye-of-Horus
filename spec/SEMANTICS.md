# Eye of Horus — Operational Semantics

**Version:** 0.1.0-draft
**Companion to:** [`LANGUAGE_SPEC.md`](LANGUAGE_SPEC.md)

---

## 1. Purpose

This document gives a small-step operational semantics for the executable
core of Eye of Horus — the subset actually dispatched by `eoh-vm`. It is
written for readers with some background in programming-language theory
(e.g. familiarity with SOS-style judgments), but keeps notation lightweight.

## 2. Domains

```
Coord   ∈ ℝ³                                     (validated: finite, |c| ≤ MAX_COORD)
Addr    ∈ ℤ³                                     (phi-pi lattice address)
Value   ::= Float(f) | Bool(b) | Str(s) | Coord(c) | Unit
Field   : Addr ⇀ Value                           (partial map — the spatial store)
Pulse   ::= ⟨origin: Coord, velocity: ℝ, birth: ℕ⟩
Config  ::= ⟨IP, Stack, Field, Pulses, tick⟩      (machine configuration)
```

The **phi-pi address function** `α : Coord → Addr` is defined as:

```
α(x, y, z) = ( round(x / q), round(y / q), round(z / q) )   where q = φ/π
```

This is a deterministic, many-to-one quantisation — multiple nearby
coordinates may map to the same address. This is intentional: it is a
spatial hashing scheme for VM storage locality, **not** a lossless
coordinate encoding and **not** a cryptographic construction.

## 3. Pulse propagation

Given a pulse `p = ⟨o, v, b⟩` and the current tick `t`:

```
radius(p, t) = max(0, t - b) · v

activates(p, point, t)  ⟺  ‖o - point‖₂ ≤ radius(p, t)
```

The activation field `Φ` at tick `t` is the union over all live pulses:

```
active(Φ, point, t)  ⟺  ∃ p ∈ Φ. activates(p, point, t)
```

**Monotonicity property.** For a fixed pulse `p` and point `x`, once
`activates(p, x, t)` holds for some `t`, it holds for all `t' ≥ t` (since
`radius` is non-decreasing in `t` for `v ≥ 0`). This means activation is
"sticky" — a shape that has been reached by a pulse wavefront remains
activated for the rest of the simulation. This is a deliberate simplification
for v0.1; transient/pulse-width-bounded activation is noted as a future
extension in the whitepaper.

## 4. Small-step reduction rules

We write `⟨IP, S, F, Φ, t⟩ → ⟨IP', S', F', Φ', t'⟩` for one VM step. Every
rule implicitly increments `t' = t + 1` (the reference implementation
advances the tick counter once per dispatched instruction — a simplification
that conflates "simulation time" with "instruction count"; see Open Problems
in the whitepaper for discussion of decoupling these).

**PushFloat**
```
image[IP] = PushFloat(f)
─────────────────────────────────────────────
⟨IP, S, F, Φ, t⟩ → ⟨IP+1, f::S, F, Φ, t+1⟩
```

**Arithmetic (Add shown; Sub/Mul analogous)**
```
image[IP] = Add        S = a::b::S'
─────────────────────────────────────────────
⟨IP, S, F, Φ, t⟩ → ⟨IP+1, (a+b)::S', F, Φ, t+1⟩
```

**Div (partial — faults on zero divisor)**
```
image[IP] = Div     S = a::b::S'     b ≠ 0
─────────────────────────────────────────────
⟨IP, S, F, Φ, t⟩ → ⟨IP+1, (a/b)::S', F, Φ, t+1⟩

image[IP] = Div     S = a::0::S'
─────────────────────────────────────────────
⟨IP, S, F, Φ, t⟩ → 𝐟𝐚𝐮𝐥𝐭(DivisionByZero)
```

**DeclareVertex**
```
image[IP] = DeclareVertex(n)     S = x::y::z::S'     c = (x,y,z) valid
─────────────────────────────────────────────────────────────────────
⟨IP, S, F, Φ, t⟩ → ⟨IP+1, S', F[α(c) ↦ Unit], Φ, t+1⟩,   vertices[n] := c
```

(Vertex-name-to-coordinate binding `vertices` is maintained as auxiliary VM
state alongside `F`, mirroring the reference implementation's `HashMap<String, Coord3D>`.)

**Store / Load** (address-indirect through the vertex table; `origin`
defaults to `Coord::ORIGIN` if `n` is not a known vertex — see §5 for why
this default exists and its implications)
```
image[IP] = Store(n)     S = v::S'     c = vertices.get(n, ORIGIN)
─────────────────────────────────────────────────────────────────
⟨IP, S, F, Φ, t⟩ → ⟨IP+1, S', F[α(c) ↦ v], Φ, t+1⟩

image[IP] = Load(n)      c = vertices.get(n, ORIGIN)     v = F(α(c), default=Unit)
────────────────────────────────────────────────────────────────────────────────
⟨IP, S, F, Φ, t⟩ → ⟨IP+1, v::S, F, Φ, t+1⟩
```

**EmitPulse**
```
image[IP] = EmitPulse{origin: n, velocity: v}     c = vertices.get(n, ORIGIN)
──────────────────────────────────────────────────────────────────────────
⟨IP, S, F, Φ, t⟩ → ⟨IP+1, S, F, Φ ∪ {⟨c, v, t⟩}, t+1⟩
```

**Halt**
```
image[IP] = Halt
─────────────────────────────────────
⟨IP, S, F, Φ, t⟩ → ⟨HALT, S, F, Φ, t⟩     (terminal configuration)
```

## 5. Known semantic gaps (documented honestly)

- **`Load`/`Store` default-to-origin behaviour.** If `n` does not name a
  declared vertex, the reference VM currently resolves its address as
  `Coord::ORIGIN` rather than raising a runtime fault. This means an
  unbound `LET` target silently aliases the origin cell. This is almost
  certainly *not* the semantics a mature version of the language should
  have — it is flagged here as a known rough edge, tracked as
  `ROADMAP.md` Phase 1 item "strict unbound-name faulting."
- **Tick/instruction conflation**, noted above — a physically-motivated
  pulse model should probably decouple "simulation time" from "instructions
  executed," since the latter is an implementation artifact.
- **No formal proof of termination or confluence** is offered for programs
  using `LOOP`/`JumpIf` once wired up (Phase 1). This is an explicit open
  problem, not a claimed result.

These gaps are listed here specifically so the specification does not imply
guarantees the implementation does not provide.
