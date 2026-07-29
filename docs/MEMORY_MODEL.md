# Memory Model

## Status

Research draft, partially implemented. The address function described below exists in `eoh-core` and is specified formally in [`spec/SEMANTICS.md`](spec/SEMANTICS.md) §2; the broader claims and design rationale are argued at length in RFC 0001 ([`rfcs/0001-phi-pi-addressing.md`](rfcs/0001-phi-pi-addressing.md)), which this document summarizes rather than duplicates.

## The Phi-Pi Addressing Scheme

Eye of Horus stores values not by raw floating-point coordinate but by a quantized lattice address, computed as:

```text
alpha(x, y, z) = ( round(x / q), round(y / q), round(z / q) ),   q = phi / pi
```

where `phi` is the golden ratio and `pi` is the ordinary circle constant, giving `q ≈ 0.515`. This is a deterministic, many-to-one quantization: distinct but nearby coordinates can map to the same lattice cell, and address equality is exact integer comparison rather than floating-point comparison.

## Why Quantization at All

A storage layer keyed directly on raw `Coord3D` values has two practical problems. First, floating-point equality is a poor storage key: two coordinates that are "the same point" mathematically may differ in their last few bits of precision depending on the arithmetic path used to compute them, which would make otherwise-identical vertices land in different storage slots. Second, a raw hash over floating-point coordinates provides no notion of spatial proximity at the storage layer — two points a millimeter apart could land in completely unrelated hash buckets. Quantizing to a lattice fixes both problems: nearby points collapse to the same or adjacent addresses, and the resulting address is a small, well-behaved key.

## Why This Particular Quantum

The choice of `q = phi / pi` rather than a simpler constant such as `q = 1.0` rests on two stated reasons, and only one of them is an engineering claim:

1. **Thematic consistency.** The project already uses golden-ratio-based constructions elsewhere (for example, a Fibonacci-sphere point-distribution utility and a golden-angle constant in the standard library scaffold), and reusing `phi` here keeps the mathematical vocabulary of the project consistent.
2. **A speculative locality hypothesis.** Because `phi`'s continued-fraction expansion gives it well-studied equidistribution properties in one-dimensional quasi-periodic tilings, it is *conjectured* — not proved and not yet benchmarked — that reusing this constant for three-dimensional lattice quantization might reduce systematic collision clustering compared to a "rounder" quantum. This is explicitly flagged as unverified in the white paper and is not the primary justification for the scheme.

## What This Model Is Not

This is the single most important paragraph in this document. The phi-pi addressing scheme is a **deterministic, public spatial-hashing function for VM storage locality**. It is:

- **not** a cryptographic mechanism, obfuscation technique, or access-control boundary of any kind;
- **not** lossless — recovering a coordinate from its lattice address returns the lattice cell's center, not the original point, and round-tripping a coordinate through an address can move it by up to `q / sqrt(3)` in the worst case;
- **not** a proven performance improvement over simpler addressing schemes; that comparison is future benchmarking work (see [`ROADMAP.md`](ROADMAP.md) Phase 4 and [`PERFORMANCE.md`](PERFORMANCE.md)).

Any documentation, marketing copy, or third-party description that characterizes phi-pi addressing as providing security, obfuscation, or a proven performance advantage is inaccurate and should be corrected; see [`SECURITY_MODEL.md`](SECURITY_MODEL.md) for the corresponding non-goal statement from the security side.

## Existing Implementation

`eoh-core` includes a small, tested utility computing this address sequence, along with the validated `Coord3D::new` constructor that guarantees every coordinate reaching the addressing function is finite and within `MAX_COORD`, per [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §4.4.

## Planned Research

- **Locality analysis.** A controlled comparison of collision and clustering behavior between the phi-pi quantum and simpler alternatives (for example, `q = 0.5` or `q = 1.0`), across representative program shapes.
- **Deterministic allocation rules** for the spatial field as programs grow beyond trivial size, including a policy for cell collisions that does not depend on hash-map iteration order.
- **Debugging displays** that render the lattice and its occupancy alongside a program's declared geometry, feeding into the visualizer described in [`VISION.md`](VISION.md).
- **Comparison against ordinary sequential allocation** in controlled, reproducible experiments, published only once methodology and results can jointly satisfy the standard set in [`PERFORMANCE.md`](PERFORMANCE.md).

## Alternatives Considered

RFC 0001 records two alternatives considered and deferred rather than adopted: plain grid quantization with a "round" constant such as `q = 1.0`, and an octree or k-d tree spatial index in place of a flat hash lattice. Both remain open for reconsideration if future benchmarking fails to show a benefit to the phi-pi choice, or if `SpatialField`'s implementation is swapped without changing the language's surface semantics.
