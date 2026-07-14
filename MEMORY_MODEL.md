# Memory Model

Status: research draft.

Eye of Horus explores a phi-pi addressing scheme:

```text
M(n) = floor(n * phi * pi)
```

This is deterministic and public. It is not a cryptographic mechanism and must not be described as a security boundary.

## Existing Implementation

`eoh-core` includes a small utility for computing this address sequence.

## Planned Research

- locality analysis;
- deterministic allocation rules;
- debugging displays;
- comparison against ordinary sequential allocation in controlled experiments.
