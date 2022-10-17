#![allow(dead_code)]

/// An token with it's infomation
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token<'a> {
    /// Type of this token
    kind: TokenKind,

    /// String span which token use
    pub span: Span<'a>,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, span: Span<'a>) -> Token<'a> {
        Token { kind, span }
    }
}

/// Type of token
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    // One-character token
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,

    // Literal
    /// Identifier
    Ident,
    /// Integer
    Integer,

    // Special
    /// Invalid character
    Invalid,
}

/// Represent substring
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span<'a> {
    /// Original string for creating substring
    string: &'a str,

    /// Start index of substring
    start: usize,

    /// End index of substring
    end: usize,
}

impl<'a> Span<'a> {
    /// Create span
    pub fn new(string: &'a str, start: usize, end: usize) -> Span<'a> {
        Span { string, start, end }
    }

    /// Get substring
    pub fn as_str(&self) -> &str {
        &self.string[self.start..self.end]
    }
}
