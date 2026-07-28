//! # eoh-ast
//!
//! Abstract Syntax Tree definitions for Eye of Horus.
//!
//! Every node carries a [`Span`] for diagnostic reporting and is `serde`-
//! serialisable so the compiler can cache and inspect ASTs.

#![deny(missing_docs)]
#![deny(unsafe_code)]

use eoh_core::span::Span;
use serde::{Deserialize, Serialize};

// ── Top-level ────────────────────────────────────────────────────────────────

/// A complete parsed source file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    /// Declarations in file order.
    pub items: Vec<Item>,
    /// Source span of the whole module.
    pub span: Span,
}

/// A top-level item inside a module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    /// `ORIGIN x, y, z`
    Origin(OriginDecl),
    /// `VERTEX name x, y, z`
    Vertex(VertexDecl),
    /// `EDGE from -> to`
    Edge(EdgeDecl),
    /// A shape declaration (`SHAPE_TETRA`, `SHAPE_CUBE`, …)
    Shape(ShapeDecl),
    /// `PULSE_HIGGS origin, v=velocity`
    Pulse(PulseDecl),
    /// `FN name(params) -> ReturnType { body }`
    Function(FnDecl),
    /// `LET name = expr`
    Let(LetDecl),
    /// `IMPORT "path/to/module"`
    Import(ImportDecl),
}

impl Item {
    /// Source span of this item.
    pub fn span(&self) -> Span {
        match self {
            Self::Origin(d)   => d.span,
            Self::Vertex(d)   => d.span,
            Self::Edge(d)     => d.span,
            Self::Shape(d)    => d.span,
            Self::Pulse(d)    => d.span,
            Self::Function(d) => d.span,
            Self::Let(d)      => d.span,
            Self::Import(d)   => d.span,
        }
    }
}

// ── Declarations ─────────────────────────────────────────────────────────────

/// `ORIGIN x, y, z`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginDecl {
    /// X coordinate expression.
    pub x: Expr,
    /// Y coordinate expression.
    pub y: Expr,
    /// Z coordinate expression.
    pub z: Expr,
    /// Source span.
    pub span: Span,
}

/// `VERTEX name x, y, z`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexDecl {
    /// Programmer-visible name.
    pub name: String,
    /// X expression.
    pub x: Expr,
    /// Y expression.
    pub y: Expr,
    /// Z expression.
    pub z: Expr,
    /// Source span.
    pub span: Span,
}

/// `EDGE from -> to`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeDecl {
    /// Source vertex name.
    pub from: String,
    /// Destination vertex name.
    pub to: String,
    /// Source span.
    pub span: Span,
}

/// A shape declaration of any solid type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeDecl {
    /// Programmer-visible name for the shape.
    pub name: String,
    /// Solid variant keyword.
    pub kind: ShapeKindAst,
    /// Vertex names.
    pub vertices: Vec<String>,
    /// Optional scalar parameter (e.g. `size=0.2`).
    pub param: Option<Expr>,
    /// Source span.
    pub span: Span,
}

/// AST-level shape kind discriminant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeKindAst {
    /// `SHAPE_TETRA`
    Tetrahedron,
    /// `SHAPE_CUBE`
    Cube,
    /// `SHAPE_ICOSA`
    Icosahedron,
    /// `SHAPE_SPHERE`
    Sphere,
    /// `SHAPE_POLY`
    Polygon,
}

/// `PULSE_HIGGS origin_name, v=velocity_expr`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulseDecl {
    /// Name of the origin vertex.
    pub origin: String,
    /// Velocity expression. Defaults to 1.0 if omitted.
    pub velocity: Option<Expr>,
    /// Source span.
    pub span: Span,
}

/// `FN name(params) -> return_type { body }`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FnDecl {
    /// Function name.
    pub name: String,
    /// Parameter list.
    pub params: Vec<Param>,
    /// Return type annotation (optional; `()` if absent).
    pub return_type: Option<TypeAnnotation>,
    /// Body block.
    pub body: Block,
    /// Source span.
    pub span: Span,
}

/// A single function parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    /// Parameter name.
    pub name: String,
    /// Type annotation.
    pub ty: TypeAnnotation,
    /// Source span.
    pub span: Span,
}

/// `LET name = expr`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetDecl {
    /// Bound name.
    pub name: String,
    /// Optional type annotation.
    pub ty: Option<TypeAnnotation>,
    /// Initialiser expression.
    pub value: Expr,
    /// Source span.
    pub span: Span,
}

/// `IMPORT "path/to/module"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDecl {
    /// Module path.
    pub path: String,
    /// Source span.
    pub span: Span,
}

// ── Types ────────────────────────────────────────────────────────────────────

/// A type annotation in the source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeAnnotation {
    /// Type name (e.g. `Float`, `Vertex`, `Shape`).
    pub name: String,
    /// Source span.
    pub span: Span,
}

// ── Expressions ──────────────────────────────────────────────────────────────

/// An expression node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expr {
    /// Expression kind and payload.
    pub kind: ExprKind,
    /// Source span.
    pub span: Span,
}

/// All valid expression forms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExprKind {
    /// A floating-point literal.
    Float(f64),
    /// A boolean literal.
    Bool(bool),
    /// A string literal.
    Str(String),
    /// A name reference.
    Ident(String),
    /// A binary operation.
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    /// A unary operation.
    UnOp(UnOp, Box<Expr>),
    /// A function call.
    Call {
        /// Callee expression (usually an identifier).
        callee: Box<Expr>,
        /// Argument expressions.
        args: Vec<Expr>,
    },
    /// A block expression (sequence of statements with a tail value).
    Block(Block),
    /// `IF cond { then } ELSE { else }`
    If {
        /// Condition expression.
        cond: Box<Expr>,
        /// Block executed when the condition is true.
        then: Block,
        /// Optional block executed when the condition is false.
        else_: Option<Block>,
    },
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinOp {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `%`
    Rem,
    /// `==`
    Eq,
    /// `!=`
    Neq,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `<=`
    Le,
    /// `>=`
    Ge,
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnOp {
    /// Arithmetic negation `-`.
    Neg,
    /// Logical not `!`.
    Not,
}

// ── Statements ───────────────────────────────────────────────────────────────

/// A statement inside a block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stmt {
    /// Statement kind.
    pub kind: StmtKind,
    /// Source span.
    pub span: Span,
}

/// All valid statement forms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StmtKind {
    /// A `LET` binding.
    Let(LetDecl),
    /// An expression used as a statement (value discarded).
    Expr(Expr),
    /// `RETURN expr`
    Return(Option<Expr>),
    /// `BREAK`
    Break,
    /// `CONTINUE`
    Continue,
    /// A `LOOP` body.
    Loop(Block),
}

/// A brace-delimited sequence of statements with an optional tail expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Statements in order.
    pub stmts: Vec<Stmt>,
    /// Optional tail expression (the block's value).
    pub tail: Option<Box<Expr>>,
    /// Source span.
    pub span: Span,
}
