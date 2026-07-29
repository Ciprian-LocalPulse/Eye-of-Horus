# Syntax

Status: draft.

```eoh
ORIGIN 0.0, 0.0, 0.0
VERTEX A 1.0, 1.618, 3.141
VECTOR FLOW A -> B
SHAPE_TETRA T1 A, B, C, D
PULSE_HIGGS ORIGIN, v=1.0
```

## Existing Implementation

No full parser exists yet.

## Planned Implementation

Syntax will be stabilized through RFCs and parser tests.

## Future RFCs

- comments and whitespace;
- identifiers and namespaces;
- numeric literal grammar;
- shape declarations;
- modules and imports.
