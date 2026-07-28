//! Lexer (tokeniser) for Eye of Horus source files.

use crate::token::{Token, TokenKind};
use eoh_core::{
    error::{EohError, EohResult},
    span::Span,
};

/// Stateful lexer that walks a UTF-8 source string character by character.
pub struct Lexer<'src> {
    source:  &'src str,
    chars:   std::str::CharIndices<'src>,
    /// Byte offset of the *current* character.
    pos:     usize,
    /// Current character (lookahead).
    current: Option<char>,
    file_id: u32,
}

impl<'src> Lexer<'src> {
    /// Construct a new lexer over `source`.
    pub fn new(source: &'src str, file_id: u32) -> Self {
        let mut chars = source.char_indices();
        let (pos, current) = chars.next().map_or((0, None), |(p, c)| (p, Some(c)));
        Self { source, chars, pos, current, file_id }
    }

    // ── Primitives ───────────────────────────────────────────────────────

    fn advance(&mut self) -> Option<char> {
        let prev = self.current;
        if let Some((p, c)) = self.chars.next() {
            self.pos = p;
            self.current = Some(c);
        } else {
            self.pos = self.source.len();
            self.current = None;
        }
        prev
    }

    fn span_from(&self, start: usize) -> Span {
        Span::new(start, self.pos, self.file_id)
    }

    fn span_at(&self, start: usize, end: usize) -> Span {
        Span::new(start, end, self.file_id)
    }

    fn lex_error(&self, msg: impl Into<String>) -> EohError {
        EohError::Lex {
            location: format!("byte {}", self.pos),
            message:  msg.into(),
        }
    }

    // ── Skip whitespace and comments ─────────────────────────────────────

    fn skip_whitespace(&mut self) {
        while matches!(self.current, Some(c) if c.is_whitespace()) {
            self.advance();
        }
    }

    // ── Numeric literals ─────────────────────────────────────────────────

    fn lex_number(&mut self, start: usize) -> EohResult<Token> {
        while matches!(self.current, Some(c) if c.is_ascii_digit() || c == '.') {
            self.advance();
        }
        // Handle scientific notation
        if matches!(self.current, Some('e') | Some('E')) {
            self.advance();
            if matches!(self.current, Some('+') | Some('-')) {
                self.advance();
            }
            while matches!(self.current, Some(c) if c.is_ascii_digit()) {
                self.advance();
            }
        }
        let end = self.pos;
        let text = &self.source[start..end];
        let value: f64 = text.parse().map_err(|_| {
            self.lex_error(format!("invalid numeric literal '{text}'"))
        })?;
        Ok(Token::new(TokenKind::Float(value), self.span_at(start, end)))
    }

    // ── String literals ──────────────────────────────────────────────────

    fn lex_string(&mut self, start: usize) -> EohResult<Token> {
        self.advance(); // consume opening `"`
        let mut buf = String::new();
        loop {
            match self.current {
                None => return Err(self.lex_error("unterminated string literal")),
                Some('"') => { self.advance(); break; }
                Some('\\') => {
                    self.advance();
                    match self.current {
                        Some('n')  => { buf.push('\n'); self.advance(); }
                        Some('t')  => { buf.push('\t'); self.advance(); }
                        Some('\\') => { buf.push('\\'); self.advance(); }
                        Some('"')  => { buf.push('"');  self.advance(); }
                        Some(c)    => return Err(self.lex_error(format!("unknown escape \\{c}"))),
                        None       => return Err(self.lex_error("unterminated escape")),
                    }
                }
                Some(c) => { buf.push(c); self.advance(); }
            }
        }
        Ok(Token::new(TokenKind::StringLit(buf), self.span_from(start)))
    }

    // ── Line comment ─────────────────────────────────────────────────────

    fn lex_comment(&mut self, start: usize) -> Token {
        self.advance(); self.advance(); // consume `//`
        let mut buf = String::new();
        while !matches!(self.current, None | Some('\n')) {
            if let Some(c) = self.advance() { buf.push(c); }
        }
        Token::new(TokenKind::Comment(buf.trim().to_owned()), self.span_from(start))
    }

    // ── Identifier / keyword ─────────────────────────────────────────────

    fn lex_ident(&mut self, start: usize) -> Token {
        while matches!(self.current, Some(c) if c.is_alphanumeric() || c == '_') {
            self.advance();
        }
        let end = self.pos;
        let text = &self.source[start..end];
        let kind = Self::keyword(text)
            .unwrap_or_else(|| match text {
                "true"  => TokenKind::Bool(true),
                "false" => TokenKind::Bool(false),
                _       => TokenKind::Ident(text.to_owned()),
            });
        Token::new(kind, self.span_at(start, end))
    }

    fn keyword(s: &str) -> Option<TokenKind> {
        Some(match s {
            "ORIGIN"       => TokenKind::KwOrigin,
            "VERTEX"       => TokenKind::KwVertex,
            "EDGE"         => TokenKind::KwEdge,
            "SHAPE_TETRA"  => TokenKind::KwShapeTetra,
            "SHAPE_CUBE"   => TokenKind::KwShapeCube,
            "SHAPE_ICOSA"  => TokenKind::KwShapeIcosa,
            "SHAPE_SPHERE" => TokenKind::KwShapeSphere,
            "PULSE_HIGGS"  => TokenKind::KwPulseHiggs,
            "LET"          => TokenKind::KwLet,
            "FN"           => TokenKind::KwFn,
            "RETURN"       => TokenKind::KwReturn,
            "IF"           => TokenKind::KwIf,
            "ELSE"         => TokenKind::KwElse,
            "LOOP"         => TokenKind::KwLoop,
            "BREAK"        => TokenKind::KwBreak,
            "CONTINUE"     => TokenKind::KwContinue,
            "IMPORT"       => TokenKind::KwImport,
            _              => return None,
        })
    }

    // ── Two-character operators ───────────────────────────────────────────

    fn lex_op(&mut self, start: usize) -> EohResult<Token> {
        let first = self.advance().unwrap();
        let kind = match (first, self.current) {
            ('-', Some('>')) => { self.advance(); TokenKind::Arrow }
            ('=', Some('=')) => { self.advance(); TokenKind::Eq }
            ('!', Some('=')) => { self.advance(); TokenKind::Neq }
            ('<', Some('=')) => { self.advance(); TokenKind::Le }
            ('>', Some('=')) => { self.advance(); TokenKind::Ge }
            ('<', _)         => TokenKind::Lt,
            ('>', _)         => TokenKind::Gt,
            ('=', _)         => TokenKind::Assign,
            ('+', _)         => TokenKind::Plus,
            ('-', _)         => TokenKind::Minus,
            ('*', _)         => TokenKind::Star,
            ('/', _)         => TokenKind::Slash,
            ('%', _)         => TokenKind::Percent,
            (',', _)         => TokenKind::Comma,
            ('.', _)         => TokenKind::Dot,
            (':', _)         => TokenKind::Colon,
            (';', _)         => TokenKind::Semicolon,
            ('(', _)         => TokenKind::LParen,
            (')', _)         => TokenKind::RParen,
            ('{', _)         => TokenKind::LBrace,
            ('}', _)         => TokenKind::RBrace,
            ('[', _)         => TokenKind::LBracket,
            (']', _)         => TokenKind::RBracket,
            (c, _)           => return Err(self.lex_error(format!("unexpected character '{c}'"))),
        };
        Ok(Token::new(kind, self.span_from(start)))
    }

    // ── Public API ───────────────────────────────────────────────────────

    /// Advance and return the next token.
    pub fn next_token(&mut self) -> EohResult<Token> {
        self.skip_whitespace();
        let start = self.pos;

        match self.current {
            None => Ok(Token::new(TokenKind::Eof, self.span_from(start))),
            Some('/') if matches!(self.chars.clone().next(), Some((_, '/'))) => {
                // Peek reveals `//` — line comment
                Ok(self.lex_comment(start))
            }
            Some(c) if c.is_ascii_digit() => self.lex_number(start),
            Some('"') => self.lex_string(start),
            Some(c) if c.is_alphabetic() || c == '_' => Ok(self.lex_ident(start)),
            _ => self.lex_op(start),
        }
    }

    /// Collect all tokens into a `Vec`, stopping after `Eof`.
    pub fn collect_all(mut self) -> EohResult<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            let is_eof = tok.is_eof();
            tokens.push(tok);
            if is_eof { break; }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(src: &str) -> Vec<TokenKind> {
        Lexer::new(src, 0).collect_all().unwrap().into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn lex_origin_keyword() {
        let kinds = lex("ORIGIN");
        assert_eq!(kinds[0], TokenKind::KwOrigin);
    }

    #[test]
    fn lex_vertex_declaration() {
        let kinds = lex("VERTEX A 1.0, 2.0, 3.0");
        assert_eq!(kinds[0], TokenKind::KwVertex);
        assert_eq!(kinds[1], TokenKind::Ident("A".to_owned()));
        assert!(matches!(kinds[2], TokenKind::Float(v) if (v - 1.0).abs() < f64::EPSILON));
    }

    #[test]
    fn lex_comment_skipped_content() {
        let kinds = lex("// hello world");
        assert!(matches!(&kinds[0], TokenKind::Comment(c) if c == "hello world"));
    }

    #[test]
    fn lex_string_literal() {
        let kinds = lex(r#""Eye of Horus""#);
        assert!(matches!(&kinds[0], TokenKind::StringLit(s) if s == "Eye of Horus"));
    }

    #[test]
    fn lex_boolean() {
        let kinds = lex("true false");
        assert_eq!(kinds[0], TokenKind::Bool(true));
        assert_eq!(kinds[1], TokenKind::Bool(false));
    }

    #[test]
    fn lex_arrow() {
        let kinds = lex("->");
        assert_eq!(kinds[0], TokenKind::Arrow);
    }
}
