# RFC 0001: Phi-Pi Addressing for the Spatial Field

- **Status:** Accepted (implemented in v0.1)
- **Author:** Ciprian Stefan Plesca
- **Created:** 2026 (project inception)

## Summary

Adopt `α(x,y,z) = (round(x/q), round(y/q), round(z/q))`, `q = φ/π`, as the
deterministic quantisation function mapping continuous `Coord3D` values to
discrete `PhiPiAddress` lattice keys used by `SpatialField<V>`.

## Motivation

Eye of Horus needs *some* way to turn continuous 3-D coordinates into a
finite, hashable key space for VM storage. A `HashMap<Coord3D, V>` keyed on
raw floating-point coordinates is possible but has two drawbacks: (a)
floating-point equality is a poor storage key (two "logically identical"
coordinates computed via different arithmetic paths may differ in the last
few ULPs), and (b) it provides no notion of *spatial proximity* at the
storage layer — two nearby-but-not-identical points get completely
unrelated hash buckets.

A quantisation lattice fixes both: nearby points collapse to the same or
adjacent addresses, and address equality is exact integer comparison.

## Detailed design

See `spec/LANGUAGE_SPEC.md` §2.3 note on lexical negative-number handling
(unrelated) and, more relevantly, `spec/SEMANTICS.md` §2 for the formal
definition. The quantum `q = φ/π ≈ 0.515` was chosen for two reasons:

1. **Thematic consistency** — the project's broader golden-ratio/spiral
   motifs (see `eoh_stdlib::math::fibonacci_sphere`, `golden_angle`) make φ
   a natural constant to reuse here.
2. **Irrationality** — φ's continued-fraction expansion `[1;1,1,1,...]`
   gives it well-studied equidistribution properties in one-dimensional
   quasi-periodic tilings (Fibonacci word / golden-ratio sequences). We
   *speculate*, but have not proven or benchmarked, that reusing this
   constant for 3-D lattice quantisation might reduce systematic collision
   clustering compared to a "rounder" quantum like `q = 0.5`. This is
   explicitly flagged as unverified in the whitepaper (§8.3) and is not the
   primary justification for adopting the scheme.

## Drawbacks

- The choice of `q = φ/π` specifically (as opposed to any other
  irrational quantum) is not derived from a locality proof — reason (2)
  above is speculative. A future RFC could revisit this if benchmarking
  (Roadmap Phase 4) shows no measurable benefit over a simpler quantum.
- Lattice quantisation is inherently lossy: `PhiPiAddress::to_coord` recovers
  the lattice-*centre*, not the original coordinate. Round-tripping a
  coordinate through an address and back can move it by up to `q/√3` in the
  worst case (half the lattice cell's space diagonal). This is by design
  but must be understood by anyone reasoning about program behaviour near
  cell boundaries.
- The name "phi-pi" and use of φ invites (unwarranted) association with
  pseudo-scientific or numerology-adjacent uses of the golden ratio
  elsewhere on the internet. We address this directly in the whitepaper
  (§4.2.1) with an explicit non-claims statement.

## Alternatives considered

- **Plain grid quantisation** (`q = 1.0` or similar "round" constant):
  simpler, no thematic tie-in, no speculative locality claim to defend.
  Rejected mainly for thematic/aesthetic reasons — this is a legitimate
  trade-off to revisit if empirical benchmarking (Phase 4) shows no
  benefit to the φ/π choice.
- **Octree / k-d tree spatial index** instead of a flat hash lattice: more
  sophisticated, handles non-uniform point density better, but
  significantly more implementation complexity for a v0.1 research
  language. Deferred — could be revisited as a `SpatialField` backend
  swap without changing the language's surface semantics.

## Unresolved questions

- Does the φ/π quantum actually outperform simpler quanta on realistic Eye
  of Horus programs? (Tracked: Roadmap Phase 4 benchmarking.)
- Should `SpatialField` support configurable quantum granularity per-program
  (e.g. via a `PRECISION` directive), or should φ/π remain a fixed language
  constant? Currently fixed; revisiting this is out of scope for v0.1.
