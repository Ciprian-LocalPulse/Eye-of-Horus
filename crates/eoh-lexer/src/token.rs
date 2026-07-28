//! Token types produced by the Eye of Horus lexer.

use eoh_core::span::Span;

/// Every distinct kind of lexical unit in the Eye of Horus grammar.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // ── Keywords ──────────────────────────────────────────────────────────
    /// `ORIGIN` — declare the coordinate origin.
    KwOrigin,
    /// `VERTEX` — declare a named vertex.
    KwVertex,
    /// `EDGE` — declare a directed edge.
    KwEdge,
    /// `SHAPE_TETRA` — declare a tetrahedron shape.
    KwShapeTetra,
    /// `SHAPE_CUBE` — declare a cube shape.
    KwShapeCube,
    /// `SHAPE_ICOSA` — declare an icosahedron.
    KwShapeIcosa,
    /// `SHAPE_SPHERE` — declare a sphere.
    KwShapeSphere,
    /// `PULSE_HIGGS` — emit a Higgs pulse.
    KwPulseHiggs,
    /// `LET` — bind a name to a value.
    KwLet,
    /// `FN` — declare a spatial function.
    KwFn,
    /// `RETURN` — return a value from a function.
    KwReturn,
    /// `IF` — conditional branch.
    KwIf,
    /// `ELSE` — alternate branch.
    KwElse,
    /// `LOOP` — unconditional loop.
    KwLoop,
    /// `BREAK` — break out of a loop.
    KwBreak,
    /// `CONTINUE` — continue to the next loop iteration.
    KwContinue,
    /// `IMPORT` — import another `.eoh` module.
    KwImport,

    // ── Literals ─────────────────────────────────────────────────────────
    /// A floating-point or integer numeric literal.
    Float(f64),
    /// A string literal (content without surrounding quotes).
    StringLit(String),
    /// A boolean literal `true` or `false`.
    Bool(bool),

    // ── Identifiers ──────────────────────────────────────────────────────
    /// Any user-defined identifier.
    Ident(String),

    // ── Punctuation ──────────────────────────────────────────────────────
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `:`
    Colon,
    /// `;`
    Semicolon,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `=`
    Assign,
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
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `->`
    Arrow,

    // ── Structural ───────────────────────────────────────────────────────
    /// A comment (content without the leading `//`).
    Comment(String),
    /// End of file.
    Eof,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KwOrigin     => write!(f, "ORIGIN"),
            Self::KwVertex     => write!(f, "VERTEX"),
            Self::KwEdge       => write!(f, "EDGE"),
            Self::KwShapeTetra => write!(f, "SHAPE_TETRA"),
            Self::KwShapeCube  => write!(f, "SHAPE_CUBE"),
            Self::KwShapeIcosa => write!(f, "SHAPE_ICOSA"),
            Self::KwShapeSphere=> write!(f, "SHAPE_SPHERE"),
            Self::KwPulseHiggs => write!(f, "PULSE_HIGGS"),
            Self::KwLet        => write!(f, "LET"),
            Self::KwFn         => write!(f, "FN"),
            Self::KwReturn     => write!(f, "RETURN"),
            Self::KwIf         => write!(f, "IF"),
            Self::KwElse       => write!(f, "ELSE"),
            Self::KwLoop       => write!(f, "LOOP"),
            Self::KwBreak      => write!(f, "BREAK"),
            Self::KwContinue   => write!(f, "CONTINUE"),
            Self::KwImport     => write!(f, "IMPORT"),
            Self::Float(v)     => write!(f, "{v}"),
            Self::StringLit(s) => write!(f, "\"{s}\""),
            Self::Bool(b)      => write!(f, "{b}"),
            Self::Ident(i)     => write!(f, "{i}"),
            Self::Comma        => write!(f, ","),
            Self::Dot          => write!(f, "."),
            Self::Colon        => write!(f, ":"),
            Self::Semicolon    => write!(f, ";"),
            Self::LParen       => write!(f, "("),
            Self::RParen       => write!(f, ")"),
            Self::LBrace       => write!(f, "{{"),
            Self::RBrace       => write!(f, "}}"),
            Self::LBracket     => write!(f, "["),
            Self::RBracket     => write!(f, "]"),
            Self::Assign       => write!(f, "="),
            Self::Eq           => write!(f, "=="),
            Self::Neq          => write!(f, "!="),
            Self::Lt           => write!(f, "<"),
            Self::Gt           => write!(f, ">"),
            Self::Le           => write!(f, "<="),
            Self::Ge           => write!(f, ">="),
            Self::Plus         => write!(f, "+"),
            Self::Minus        => write!(f, "-"),
            Self::Star         => write!(f, "*"),
            Self::Slash        => write!(f, "/"),
            Self::Percent      => write!(f, "%"),
            Self::Arrow        => write!(f, "->"),
            Self::Comment(c)   => write!(f, "// {c}"),
            Self::Eof          => write!(f, "<EOF>"),
        }
    }
}

/// A lexical token with source-location information.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The token's kind and payload.
    pub kind: TokenKind,
    /// Source location of this token.
    pub span: Span,
}

impl Token {
    /// Construct a token.
    pub fn new(kind: TokenKind, span: Span) -> Self { Self { kind, span } }

    /// `true` if this token is the end-of-file sentinel.
    pub fn is_eof(&self) -> bool { matches!(self.kind, TokenKind::Eof) }
}
