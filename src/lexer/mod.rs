#![allow(dead_code)]

use std::str::CharIndices;
use self::token::{
    Span,
    Token,
    TokenKind,
};
use crate::utils::iter::{
    MorePeekable,
    MorePeekableIter,
};

pub mod token;

pub struct Lexer<'a> {
    input: &'a str,
    chars: MorePeekableIter<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { input, chars: input.char_indices().more_peekable() }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_while(|c| c.is_ascii_whitespace());
        let curr = self.peek_curr_char()?;
        let next = self.peek_next_char().unwrap_or('\0');
        match (curr, next) {
            // One character
            ('+', _) => self.trim_start_one(TokenKind::Plus),
            ('-', _) => self.trim_start_one(TokenKind::Minus),
            ('*', _) => self.trim_start_one(TokenKind::Star),
            ('/', _) => self.trim_start_one(TokenKind::Slash),

            // Identifier
            (c, _) if c.is_ascii_alphabetic() => self.trim_start_with(TokenKind::Ident, |c| {
                c.is_ascii_alphanumeric() || c == '_'
            }),

            // Integer
            (c, _) if c.is_ascii_digit() => self.trim_start_with(TokenKind::Integer, |c| {
                c.is_ascii_digit()
            }),

            // Invalid
            _ => self.trim_start_one(TokenKind::Invalid),
        }
    }
}

impl<'a> Lexer<'a> {
    fn peek_curr_offset(&self) -> Option<usize> {
        Some(self.chars.peek_curr()?.0)
    }

    fn peek_curr_char(&self) -> Option<char> {
        Some(self.chars.peek_curr()?.1)
    }

    fn peek_next_offset(&self) -> Option<usize> {
        Some(self.chars.peek_next()?.0)
    }

    fn peek_next_char(&self) -> Option<char> {
        Some(self.chars.peek_next()?.1)
    }

    fn next(&mut self) {
        self.chars.next();
    }

    /// Go to next item while cond(c) return true
    fn next_while<F: Fn(char) -> bool>(&mut self, cond: F) {
        while let Some(c) = self.peek_curr_char() {
            if cond(c) {
                self.next();
            } else {
                break;
            }
        }
    }

    /// Create a token which have one character
    fn trim_start_one(&mut self, kind: TokenKind) -> Option<Token<'a>> {
        let span = {
            let start = self.peek_curr_offset()?;
            let end = self.peek_next_offset()?;
            self.next();
            Span::new(&self.input, start, end)
        };
        Some(Token::new(kind, span))
    }

    /// Create a token which have two character
    fn trim_start_two(&mut self, kind: TokenKind) -> Option<Token<'a>> {
        let span = {
            let start = self.peek_curr_offset()?;
            self.next();
            let end = self.peek_next_offset()?;
            self.next();
            Span::new(&self.input, start, end)
        };
        Some(Token::new(kind, span))
    }

    /// Trim a Span where all element is true, then create a token
    /// using this substring and given TokenKind.
    fn trim_start_with<F>(&mut self, kind: TokenKind, cond: F) -> Option<Token<'a>>
    where
        F: Fn(char) -> bool
    {
        let span = {
            let start = self.peek_curr_offset()?;
            self.next_while(cond);
            let end = self.peek_curr_offset().unwrap_or(self.input.len());
            Span::new(&self.input, start, end)
        };
        Some(Token::new(kind, span))
    }
}
