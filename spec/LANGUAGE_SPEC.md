# Eye of Horus — Language Specification

**Version:** 0.1.0-draft
**Status:** Living document, tracks the reference implementation in `crates/`
**Author:** Ciprian Stefan Plesca

---

## 1. Scope

This document specifies the surface syntax, static semantics, and
instruction-set of Eye of Horus (`.eoh`) as implemented by the reference
toolchain in this repository. Where the implementation and this document
disagree, that is a bug — please file an issue.

This is **not** a standard in the ISO/ECMA sense; it is a single-implementation
research language specification, versioned alongside the code.

## 2. Lexical structure

### 2.1 Character set

Source files are UTF-8. Identifiers are restricted to ASCII alphanumerics and
underscore for now (`[A-Za-z_][A-Za-z0-9_]*`); Unicode identifier support is
tracked in `ROADMAP.md` Phase 2.

### 2.2 Whitespace and comments

Whitespace (space, tab, newline, CR) is insignificant except as a token
separator. Comments begin with `//` and run to end of line. There are no
block comments in v0.1.

### 2.3 Tokens

```
keyword     ::= "ORIGIN" | "VERTEX" | "EDGE"
              | "SHAPE_TETRA" | "SHAPE_CUBE" | "SHAPE_ICOSA" | "SHAPE_SPHERE"
              | "PULSE_HIGGS"
              | "LET" | "FN" | "RETURN"
              | "IF" | "ELSE" | "LOOP" | "BREAK" | "CONTINUE"
              | "IMPORT"

ident       ::= [A-Za-z_][A-Za-z0-9_]*
float       ::= digit+ ("." digit+)? (("e"|"E") ("+"|"-")? digit+)?
string      ::= '"' ( [^"\\] | '\\' escape )* '"'
escape      ::= 'n' | 't' | '\\' | '"'
bool        ::= "true" | "false"

punct       ::= "," | "." | ":" | ";"
              | "(" | ")" | "{" | "}" | "[" | "]"
              | "=" | "==" | "!=" | "<" | ">" | "<=" | ">="
              | "+" | "-" | "*" | "/" | "%" | "->"
```

Negative numeric literals are **not** a distinct lexical class: `-1.0` lexes
as `Minus` followed by `Float(1.0)`, and unary negation is resolved by the
parser (`parse_unary` / `parse_float_expr`). This keeps the lexer
context-free and avoids ambiguity with binary subtraction.

## 3. Grammar (EBNF)

```ebnf
module        = { comment | item } ;

item          = origin_decl
              | vertex_decl
              | edge_decl
              | shape_decl
              | pulse_decl
              | let_decl
              | fn_decl
              | import_decl ;

origin_decl   = "ORIGIN" float_expr "," float_expr "," float_expr ;

vertex_decl   = "VERTEX" ident float_expr "," float_expr "," float_expr ;

edge_decl     = "EDGE" ident "->" ident ;

shape_decl    = shape_kw ident vertex_list [ "," param_name "=" float_expr ] ;
shape_kw      = "SHAPE_TETRA" | "SHAPE_CUBE" | "SHAPE_ICOSA" | "SHAPE_SPHERE" ;
vertex_list   = ident { "," ident } ;
param_name    = ident ;   (* e.g. "size", "r" *)

pulse_decl    = "PULSE_HIGGS" ident [ "," "v" "=" float_expr ] ;

let_decl      = "LET" ident [ ":" type_annotation ] "=" expr ;

fn_decl       = "FN" ident "(" [ param { "," param } ] ")"
                [ "->" type_annotation ] block ;
param         = ident ":" type_annotation ;

import_decl   = "IMPORT" string ;

type_annotation = ident ;   (* e.g. Float, Bool, Str, Vertex, Shape *)

block         = "{" { stmt } "}" ;
stmt          = let_decl
              | "RETURN" [ expr ] [ ";" ]
              | "BREAK" [ ";" ]
              | "CONTINUE" [ ";" ]
              | expr [ ";" ] ;

expr          = cmp_expr ;
cmp_expr      = add_expr { cmp_op add_expr } ;
cmp_op        = "==" | "!=" | "<" | ">" | "<=" | ">=" ;
add_expr      = mul_expr { ("+" | "-") mul_expr } ;
mul_expr      = unary_expr { ("*" | "/" | "%") unary_expr } ;
unary_expr    = "-" unary_expr | primary_expr ;
primary_expr  = float | bool | string | ident [ call_args ] | "(" expr ")" ;
call_args     = "(" [ expr { "," expr } ] ")" ;

float_expr    = [ "-" ] ( float | ident ) ;   (* used in coordinate positions *)
```

## 4. Static semantics

### 4.1 Name resolution

Every `VERTEX`, `FN`, `LET`, and `SHAPE_*` declaration introduces a binding
into module scope (v0.1 has no nested lexical scoping beyond function
bodies — this is tracked as a Phase 2 item). References that do not resolve
to a prior declaration are a resolution error.

### 4.2 Shape vertex-count constraints

| Shape kind | Required vertex count |
|---|---|
| `SHAPE_TETRA` | exactly 4 |
| `SHAPE_ICOSA` | exactly 12 |
| `SHAPE_CUBE` | 1 (anchor) + `size` parameter |
| `SHAPE_SPHERE` | 1 (center) + `r` parameter |
| `SHAPE_POLY` | ≥ 3 |

Violating these constraints is a **geometry error**, reported before code
generation.

### 4.3 Type system (current subset)

Eye of Horus v0.1 has a minimal, mostly-structural type system:

- `Float` — IEEE-754 double, validated finite and within `MAX_COORD` for
  spatial use.
- `Bool`
- `Str`
- `Vertex` — a named point; implicit type of `VERTEX` declarations.
- `Shape` — implicit type of `SHAPE_*` declarations.

Full Hindley–Milner-style inference across function boundaries is **not
yet implemented**; today's checker performs structural validation only
(vertex-count checks, `PULSE_HIGGS` origin existence). This is documented
honestly in `ROADMAP.md` Phase 2 and in the whitepaper's Open Problems
section.

### 4.4 Coordinate domain constraints

All coordinate components must be finite (`!NaN`, `!Inf`) and satisfy
`|component| <= MAX_COORD` (1,000,000.0 in the reference implementation).
Violations raise `EohError::Geometry` at construction time — this is
enforced at the `Coord3D::new` boundary in `eoh-core`, so it is impossible
to construct an invalid coordinate anywhere in the toolchain.

## 5. Operational model — the pulse-activation semantics

See [`SEMANTICS.md`](SEMANTICS.md) for the full operational semantics. In
summary:

1. A `PULSE_HIGGS origin, v=velocity` instruction creates a `Pulse` with
   `birth_tick = current_tick`, expanding isotropically from `origin` at
   `velocity` spatial units per simulation tick.
2. At simulation tick `t`, the pulse's wavefront radius is
   `r(t) = (t - birth_tick) * velocity`.
3. A point `p` is **activated** by a pulse at tick `t` iff
   `distance(origin, p) <= r(t)`.
4. An `ActivationField` is the union of all live pulses; a point is active
   if *any* pulse activates it (`ActivationField::is_active`).

This model is intentionally simple in v0.1 — directional pulses
(`PulseVector` beyond the isotropic case) and pulse interference/cancellation
are specified in the whitepaper as future extensions but not yet load-bearing
in the VM.

## 6. Bytecode instruction set

The reference compiler emits a flat instruction list (`eoh_compiler::bytecode::Instruction`).
This is a **stack-based** bytecode:

| Instruction | Stack effect | Description |
|---|---|---|
| `PushFloat(f)` | `→ f` | Push a float constant |
| `PushBool(b)` | `→ b` | Push a bool constant |
| `PushStr(i)` | `→ s` | Push interned string at pool index `i` |
| `Load(name)` | `→ v` | Load value bound to `name` from the spatial field |
| `Store(name)` | `v →` | Store top-of-stack into the spatial field at `name`'s address |
| `Add` / `Sub` / `Mul` / `Div` | `a b → r` | Arithmetic; `Div` faults on zero divisor |
| `DeclareVertex(name)` | `x y z →` | Bind `name` to coordinate `(x,y,z)` |
| `DeclareShape{..}` | — | Register shape metadata |
| `EmitPulse{origin,velocity}` | — | Create and register a `Pulse` |
| `Call{name,argc}` | `a1..aN →` | Invoke built-in or user function |
| `Return` | — | Halt the current frame |
| `Jump(target)` | — | Unconditional jump |
| `JumpIf(target)` | `b →` | Conditional jump (pop bool) |
| `Halt` | — | Stop VM execution |

Every emitted image is terminated with an implicit `Halt`. See
`eoh_compiler::bytecode::BytecodeImage` for the serialisable envelope
(`instructions`, `strings`, `source_path`, `version`).

## 7. Bytecode schema versioning

`BytecodeImage::version` (currently `1`) is bumped on any breaking change to
the `Instruction` enum's serialised shape. Tooling should reject images with
an unrecognised version rather than attempt best-effort execution.

## 8. Reserved for future revisions

The following are named and reserved in the grammar/keyword table but not
yet load-bearing in the VM — tracked in `ROADMAP.md`:

- `IF` / `ELSE` conditional execution (parses; MIR lowering for blocks with
  branches is Phase 1)
- `LOOP` / `BREAK` / `CONTINUE` iteration (parses; VM dispatch is Phase 1)
- `IMPORT` module resolution across files (parses; multi-file linking is
  Phase 2)
- `SHAPE_POLY` (arbitrary polygon) full geometry support beyond vertex-count
  validation
