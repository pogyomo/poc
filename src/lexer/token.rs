/// An token with it's infomation
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token<'a> {
    /// Type of this token
    kind: TokenKind,

    /// String span which token use
    span: Span<'a>,
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

    // Two-character token
    /// ==
    EQ,
    /// !=
    NE,

    // Literal
    /// Identifier
    Ident,
    /// Integer
    Integer(IntBase),

    // Special
    /// Invalid character
    Invalid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntBase {
    Binary,
    Octadecimal,
    Decimal,
    Hexadecimal,
}

impl From<IntBase> for u32 {
    fn from(base: IntBase) -> Self {
        use IntBase::*;
        match base {
            Binary      => 02,
            Octadecimal => 08,
            Decimal     => 10,
            Hexadecimal => 16,
        }
    }
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
}
