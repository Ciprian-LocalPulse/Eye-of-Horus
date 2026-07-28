# Runtime

## Status

Planned; specified at the level of formal semantics ahead of a complete standalone implementation. This document describes the engineering responsibilities of the runtime; the precise operational rules it must implement are given in [`spec/SEMANTICS.md`](spec/SEMANTICS.md).

## Responsibilities

The runtime is expected to:

1. **Simulate pulse propagation.** For each live pulse `⟨origin, velocity, birth_tick⟩`, compute its wavefront radius at the current tick as `radius(t) = max(0, t - birth_tick) * velocity`, per the formal definition already given in [`spec/SEMANTICS.md`](spec/SEMANTICS.md) §3.
2. **Schedule activation events.** Determine, at each tick, which declared points newly satisfy `distance(origin, point) <= radius(t)` for some live pulse, and emit an activation event for each.
3. **Resolve intersections.** Determine when two or more shapes, or the activation fields of two or more pulses, come to occupy or reach a shared coordinate, and emit an intersection event.
4. **Emit observable output.** Produce a structured, serializable event stream that the CLI (today) and a future visualizer (planned) can both consume without depending on internal runtime data structures.

## The Tick Model

The reference semantics currently advance the tick counter once per dispatched instruction, which conflates "simulation time" (the physical quantity that determines pulse radius) with "instruction count" (an implementation artifact of how many bytecode steps were executed to get there). This is documented as a known simplification, not a considered design choice, in [`spec/SEMANTICS.md`](spec/SEMANTICS.md) §5. A more physically motivated model would decouple these two notions of time — for example, by giving `PULSE_HIGGS` and arithmetic instructions different tick costs, or by introducing an explicit `TICK` or `ADVANCE` directive — but doing so is future work, not yet specified.

## TODO

The following runtime-level questions remain open and are tracked here so that they are not lost between the architecture-level description in [`ARCHITECTURE.md`](ARCHITECTURE.md) and the fully formal semantics:

- **Floating-point policy.** Whether pulse radius and distance computations should be specified to a particular rounding behavior (to keep results reproducible across platforms), or whether ordinary IEEE-754 double arithmetic without additional constraints is judged sufficient.
- **Deterministic sorting for simultaneous activation.** When multiple points are activated at the same tick, or multiple pulses reach the same point simultaneously, the order in which their events are emitted must be fixed and specified, not left to incidental iteration order over a hash map — this is the same open problem noted as "same-radius activation as deterministic batching" in [`ARCHITECTURE.md`](ARCHITECTURE.md#research-ideas).
- **Resource limits.** Bounds on the number of live pulses, the size of the spatial field, and the maximum tick count a single execution may run for, needed before the runtime can safely accept untrusted input (see [`SECURITY_MODEL.md`](SECURITY_MODEL.md)).
- **Tracing and debugging events.** A stable event schema rich enough to drive both a textual trace and the planned visual debugger described in [`VISION.md`](VISION.md), without requiring two separate instrumentation paths.

## Known Semantic Gap Inherited From the Specification

The `Load`/`Store` default-to-origin behavior for unbound names, documented in [`spec/SEMANTICS.md`](spec/SEMANTICS.md) §5, is a runtime behavior, not merely a specification curiosity, and any runtime implementation must currently reproduce it faithfully or explicitly deviate from the specification and say so. Resolving this — most likely by making unbound-name access a runtime fault — is tracked as a Phase 1 roadmap item.
