//! Recursive-descent parser implementation.

use eoh_ast::*;
use eoh_core::{
    error::{EohError, EohResult},
    span::Span,
};
use eoh_lexer::{Token, TokenKind};

/// Stateful parser over a token stream.
pub struct Parser {
    tokens:  Vec<Token>,
    cursor:  usize,
    file_id: u32,
}

impl Parser {
    /// Construct a parser over `tokens`.
    pub fn new(tokens: Vec<Token>, file_id: u32) -> Self {
        Self { tokens, cursor: 0, file_id }
    }

    // ── Low-level primitives ─────────────────────────────────────────────

    fn peek(&self) -> &Token {
        &self.tokens[self.cursor.min(self.tokens.len() - 1)]
    }

    fn advance(&mut self) -> &Token {
        let t = &self.tokens[self.cursor];
        if self.cursor < self.tokens.len() - 1 { self.cursor += 1; }
        t
    }

    fn expect(&mut self, kind: &TokenKind) -> EohResult<&Token> {
        let t = self.advance().clone();
        if std::mem::discriminant(&t.kind) == std::mem::discriminant(kind) {
            Ok(&self.tokens[self.cursor.saturating_sub(1)])
        } else {
            Err(self.err(format!("expected {kind}, got {}", t.kind), t.span))
        }
    }

    fn err(&self, msg: String, span: Span) -> EohError {
        EohError::Parse { location: format!("{}:{}", self.file_id, span.start), message: msg }
    }

    fn at_eof(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    // ── Module ───────────────────────────────────────────────────────────

    /// Parse a complete module.
    pub fn parse_module(&mut self) -> EohResult<Module> {
        let start = self.peek().span;
        let mut items = Vec::new();
        while !self.at_eof() {
            // Skip comments
            if matches!(self.peek().kind, TokenKind::Comment(_)) {
                self.advance();
                continue;
            }
            items.push(self.parse_item()?);
        }
        let end = self.peek().span;
        Ok(Module { items, span: start.merge(end) })
    }

    // ── Items ────────────────────────────────────────────────────────────

    fn parse_item(&mut self) -> EohResult<Item> {
        let tok = self.peek().clone();
        match &tok.kind {
            TokenKind::KwOrigin     => self.parse_origin().map(Item::Origin),
            TokenKind::KwVertex     => self.parse_vertex().map(Item::Vertex),
            TokenKind::KwEdge       => self.parse_edge().map(Item::Edge),
            TokenKind::KwShapeTetra |
            TokenKind::KwShapeCube  |
            TokenKind::KwShapeIcosa |
            TokenKind::KwShapeSphere => self.parse_shape().map(Item::Shape),
            TokenKind::KwPulseHiggs  => self.parse_pulse().map(Item::Pulse),
            TokenKind::KwLet         => self.parse_let().map(Item::Let),
            TokenKind::KwFn          => self.parse_fn().map(Item::Function),
            TokenKind::KwImport      => self.parse_import().map(Item::Import),
            _ => Err(self.err(format!("unexpected token '{}' at top level", tok.kind), tok.span)),
        }
    }

    // ── ORIGIN ───────────────────────────────────────────────────────────

    fn parse_origin(&mut self) -> EohResult<OriginDecl> {
        let start = self.peek().span;
        self.advance(); // consume ORIGIN
        let x = self.parse_float_expr()?;
        self.expect(&TokenKind::Comma)?;
        let y = self.parse_float_expr()?;
        self.expect(&TokenKind::Comma)?;
        let z = self.parse_float_expr()?;
        Ok(OriginDecl { x, y, z, span: start })
    }

    // ── VERTEX ───────────────────────────────────────────────────────────

    fn parse_vertex(&mut self) -> EohResult<VertexDecl> {
        let start = self.peek().span;
        self.advance(); // consume VERTEX
        let name = self.parse_ident()?;
        let x = self.parse_float_expr()?;
        self.expect(&TokenKind::Comma)?;
        let y = self.parse_float_expr()?;
        self.expect(&TokenKind::Comma)?;
        let z = self.parse_float_expr()?;
        Ok(VertexDecl { name, x, y, z, span: start })
    }

    // ── EDGE ─────────────────────────────────────────────────────────────

    fn parse_edge(&mut self) -> EohResult<EdgeDecl> {
        let start = self.peek().span;
        self.advance(); // consume EDGE
        let from = self.parse_ident()?;
        self.expect(&TokenKind::Arrow)?;
        let to = self.parse_ident()?;
        Ok(EdgeDecl { from, to, span: start })
    }

    // ── SHAPE ────────────────────────────────────────────────────────────

    fn parse_shape(&mut self) -> EohResult<ShapeDecl> {
        let tok = self.advance().clone();
        let kind = match &tok.kind {
            TokenKind::KwShapeTetra  => ShapeKindAst::Tetrahedron,
            TokenKind::KwShapeCube   => ShapeKindAst::Cube,
            TokenKind::KwShapeIcosa  => ShapeKindAst::Icosahedron,
            TokenKind::KwShapeSphere => ShapeKindAst::Sphere,
            _ => return Err(self.err("expected shape keyword".into(), tok.span)),
        };
        let name = self.parse_ident()?;
        let mut vertices = vec![self.parse_ident()?];
        while matches!(self.peek().kind, TokenKind::Comma) {
            self.advance(); // consume `,`
            // Allow trailing `size=` or `r=` named param
            if matches!(self.peek().kind, TokenKind::Ident(_)) {
                let nxt = self.tokens.get(self.cursor + 1);
                if matches!(nxt.map(|t| &t.kind), Some(TokenKind::Assign)) {
                    break; // named param follows — stop vertex list
                }
            }
            vertices.push(self.parse_ident()?);
        }
        // Optional named scalar param: `size=expr` or `r=expr`
        let param = if matches!(self.peek().kind, TokenKind::Ident(_)) {
            self.advance(); // consume param name
            self.expect(&TokenKind::Assign)?;
            Some(self.parse_float_expr()?)
        } else {
            None
        };
        Ok(ShapeDecl { name, kind, vertices, param, span: tok.span })
    }

    // ── PULSE_HIGGS ──────────────────────────────────────────────────────

    fn parse_pulse(&mut self) -> EohResult<PulseDecl> {
        let start = self.peek().span;
        self.advance(); // consume PULSE_HIGGS
        let origin = self.parse_ident()?;
        let velocity = if matches!(self.peek().kind, TokenKind::Comma) {
            self.advance(); // consume `,`
            // expect `v=expr`
            self.parse_ident()?; // consume `v`
            self.expect(&TokenKind::Assign)?;
            Some(self.parse_float_expr()?)
        } else {
            None
        };
        Ok(PulseDecl { origin, velocity, span: start })
    }

    // ── LET ──────────────────────────────────────────────────────────────

    fn parse_let(&mut self) -> EohResult<LetDecl> {
        let start = self.peek().span;
        self.advance(); // consume LET
        let name = self.parse_ident()?;
        let ty = if matches!(self.peek().kind, TokenKind::Colon) {
            self.advance();
            Some(self.parse_type_annotation()?)
        } else { None };
        self.expect(&TokenKind::Assign)?;
        let value = self.parse_expr()?;
        Ok(LetDecl { name, ty, value, span: start })
    }

    // ── FN ───────────────────────────────────────────────────────────────

    fn parse_fn(&mut self) -> EohResult<FnDecl> {
        let start = self.peek().span;
        self.advance(); // consume FN
        let name = self.parse_ident()?;
        self.expect(&TokenKind::LParen)?;
        let mut params = Vec::new();
        while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
            let pstart = self.peek().span;
            let pname = self.parse_ident()?;
            self.expect(&TokenKind::Colon)?;
            let pty = self.parse_type_annotation()?;
            params.push(Param { name: pname, ty: pty, span: pstart });
            if matches!(self.peek().kind, TokenKind::Comma) { self.advance(); }
        }
        self.expect(&TokenKind::RParen)?;
        let return_type = if matches!(self.peek().kind, TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type_annotation()?)
        } else { None };
        let body = self.parse_block()?;
        Ok(FnDecl { name, params, return_type, body, span: start })
    }

    // ── IMPORT ───────────────────────────────────────────────────────────

    fn parse_import(&mut self) -> EohResult<ImportDecl> {
        let start = self.peek().span;
        self.advance(); // consume IMPORT
        let tok = self.advance().clone();
        if let TokenKind::StringLit(path) = tok.kind {
            Ok(ImportDecl { path, span: start })
        } else {
            Err(self.err("expected string literal after IMPORT".into(), tok.span))
        }
    }

    // ── Blocks ───────────────────────────────────────────────────────────

    fn parse_block(&mut self) -> EohResult<Block> {
        let start = self.peek().span;
        self.expect(&TokenKind::LBrace)?;
        let mut stmts = Vec::new();
        while !matches!(self.peek().kind, TokenKind::RBrace | TokenKind::Eof) {
            stmts.push(self.parse_stmt()?);
        }
        let end = self.peek().span;
        self.expect(&TokenKind::RBrace)?;
        Ok(Block { stmts, tail: None, span: start.merge(end) })
    }

    fn parse_stmt(&mut self) -> EohResult<Stmt> {
        let start = self.peek().span;
        let kind = match &self.peek().kind {
            TokenKind::KwLet    => StmtKind::Let(self.parse_let()?),
            TokenKind::KwReturn => {
                self.advance();
                let e = if !matches!(self.peek().kind, TokenKind::Semicolon | TokenKind::RBrace) {
                    Some(self.parse_expr()?)
                } else { None };
                StmtKind::Return(e)
            }
            TokenKind::KwBreak    => { self.advance(); StmtKind::Break }
            TokenKind::KwContinue => { self.advance(); StmtKind::Continue }
            _ => StmtKind::Expr(self.parse_expr()?),
        };
        // Consume optional semicolon
        if matches!(self.peek().kind, TokenKind::Semicolon) { self.advance(); }
        Ok(Stmt { kind, span: start })
    }

    // ── Expressions ──────────────────────────────────────────────────────

    fn parse_expr(&mut self) -> EohResult<Expr> {
        self.parse_cmp()
    }

    fn parse_cmp(&mut self) -> EohResult<Expr> {
        let mut lhs = self.parse_add()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::Eq  => BinOp::Eq,
                TokenKind::Neq => BinOp::Neq,
                TokenKind::Lt  => BinOp::Lt,
                TokenKind::Gt  => BinOp::Gt,
                TokenKind::Le  => BinOp::Le,
                TokenKind::Ge  => BinOp::Ge,
                _              => break,
            };
            let span = self.peek().span;
            self.advance();
            let rhs = self.parse_add()?;
            lhs = Expr { kind: ExprKind::BinOp(Box::new(lhs), op, Box::new(rhs)), span };
        }
        Ok(lhs)
    }

    fn parse_add(&mut self) -> EohResult<Expr> {
        let mut lhs = self.parse_mul()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::Plus  => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _                => break,
            };
            let span = self.peek().span;
            self.advance();
            let rhs = self.parse_mul()?;
            lhs = Expr { kind: ExprKind::BinOp(Box::new(lhs), op, Box::new(rhs)), span };
        }
        Ok(lhs)
    }

    fn parse_mul(&mut self) -> EohResult<Expr> {
        let mut lhs = self.parse_unary()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::Star    => BinOp::Mul,
                TokenKind::Slash   => BinOp::Div,
                TokenKind::Percent => BinOp::Rem,
                _                  => break,
            };
            let span = self.peek().span;
            self.advance();
            let rhs = self.parse_unary()?;
            lhs = Expr { kind: ExprKind::BinOp(Box::new(lhs), op, Box::new(rhs)), span };
        }
        Ok(lhs)
    }

    fn parse_unary(&mut self) -> EohResult<Expr> {
        let tok = self.peek().clone();
        match &tok.kind {
            TokenKind::Minus => {
                self.advance();
                let e = self.parse_primary()?;
                Ok(Expr { kind: ExprKind::UnOp(UnOp::Neg, Box::new(e)), span: tok.span })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> EohResult<Expr> {
        let tok = self.advance().clone();
        match &tok.kind {
            TokenKind::Float(f)     => Ok(Expr { kind: ExprKind::Float(*f), span: tok.span }),
            TokenKind::Bool(b)      => Ok(Expr { kind: ExprKind::Bool(*b), span: tok.span }),
            TokenKind::StringLit(s) => Ok(Expr { kind: ExprKind::Str(s.clone()), span: tok.span }),
            TokenKind::Ident(name) => {
                // Function call?
                if matches!(self.peek().kind, TokenKind::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.peek().kind, TokenKind::RParen | TokenKind::Eof) {
                        args.push(self.parse_expr()?);
                        if matches!(self.peek().kind, TokenKind::Comma) { self.advance(); }
                    }
                    self.expect(&TokenKind::RParen)?;
                    let callee = Expr { kind: ExprKind::Ident(name.clone()), span: tok.span };
                    Ok(Expr { kind: ExprKind::Call { callee: Box::new(callee), args }, span: tok.span })
                } else {
                    Ok(Expr { kind: ExprKind::Ident(name.clone()), span: tok.span })
                }
            }
            TokenKind::LParen => {
                let e = self.parse_expr()?;
                self.expect(&TokenKind::RParen)?;
                Ok(e)
            }
            _ => Err(self.err(format!("unexpected token '{}' in expression", tok.kind), tok.span)),
        }
    }

    // ── Helpers ──────────────────────────────────────────────────────────

    fn parse_float_expr(&mut self) -> EohResult<Expr> {
        // Allow unary minus
        let tok = self.peek().clone();
        if matches!(tok.kind, TokenKind::Minus) {
            self.advance();
            let inner = self.parse_float_expr()?;
            return Ok(Expr { kind: ExprKind::UnOp(UnOp::Neg, Box::new(inner)), span: tok.span });
        }
        let tok = self.advance().clone();
        match &tok.kind {
            TokenKind::Float(f) => Ok(Expr { kind: ExprKind::Float(*f), span: tok.span }),
            TokenKind::Ident(n) => Ok(Expr { kind: ExprKind::Ident(n.clone()), span: tok.span }),
            _ => Err(self.err(format!("expected number or identifier, got '{}'", tok.kind), tok.span)),
        }
    }

    fn parse_ident(&mut self) -> EohResult<String> {
        let tok = self.advance().clone();
        if let TokenKind::Ident(name) = &tok.kind {
            Ok(name.clone())
        } else {
            Err(self.err(format!("expected identifier, got '{}'", tok.kind), tok.span))
        }
    }

    fn parse_type_annotation(&mut self) -> EohResult<TypeAnnotation> {
        let tok = self.advance().clone();
        if let TokenKind::Ident(name) = &tok.kind {
            Ok(TypeAnnotation { name: name.clone(), span: tok.span })
        } else {
            Err(self.err(format!("expected type name, got '{}'", tok.kind), tok.span))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eoh_lexer::lex;

    fn parse_src(src: &str) -> Module {
        let tokens = lex(src, 0).expect("lex failed");
        crate::parse(tokens, 0).expect("parse failed")
    }

    #[test]
    fn parse_origin() {
        let m = parse_src("ORIGIN 0.0, 0.0, 0.0");
        assert!(matches!(m.items[0], Item::Origin(_)));
    }

    #[test]
    fn parse_vertex() {
        let m = parse_src("VERTEX A 1.0, 2.0, 3.0");
        assert!(matches!(&m.items[0], Item::Vertex(v) if v.name == "A"));
    }

    #[test]
    fn parse_edge() {
        let m = parse_src("VERTEX A 0.0, 0.0, 0.0\nVERTEX B 1.0, 0.0, 0.0\nEDGE A -> B");
        assert!(matches!(&m.items[2], Item::Edge(e) if e.from == "A" && e.to == "B"));
    }

    #[test]
    fn parse_let_binding() {
        let m = parse_src("LET scale = 2.0");
        assert!(matches!(&m.items[0], Item::Let(l) if l.name == "scale"));
    }
}
