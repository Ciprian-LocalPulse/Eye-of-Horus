# Versioning

## Policy

Eye of Horus will use pre-1.0 versions for as long as the language's syntax, semantics, and compatibility guarantees remain unstable, following the convention that a `0.x` version carries no compatibility promise between minor versions. This is a deliberate application of [Design Principle 2](DESIGN_PRINCIPLES.md#2-specification-before-stabilization): a version number should not imply stability the specification has not yet earned.

## Draft Version Line

- **`0.1.x`** — repository scaffolding, formal specification drafting, and parser experiments. Corresponds to [`ROADMAP.md`](ROADMAP.md) Phase 0 and the early portion of Phase 1.
- **`0.2.x`** — executable interpreter experiments, once the language core reaches the point described in Phase 1 of the roadmap.
- **`0.3.x`** — editor tooling and stabilized examples, corresponding to Phase 2.
- **`1.0.0`** — reserved, and will not be assigned until syntax, semantics, and compatibility guarantees are formally defined and the reserved-but-not-yet-executable constructs listed in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §8 have complete execution semantics.

## What a Version Number Will Mean Once Assigned

Once the project begins tagging `0.x` releases, each tagged version will correspond to a specific, frozen snapshot of the specification documents in [`spec/`](spec/README.md), so that "which version of the language does this program target" has a precise, checkable answer via the `BytecodeImage::version` schema field described in [`spec/LANGUAGE_SPEC.md`](spec/LANGUAGE_SPEC.md) §7, once that schema itself is exercised by a tagged release.

## Relationship to Supported Versions

See [`SUPPORTED_VERSIONS.md`](SUPPORTED_VERSIONS.md) for which version lines currently receive security attention, and [`CHANGELOG.md`](CHANGELOG.md) for the record of what has changed within the `main` branch prior to the first tagged release.
