#![allow(dead_code)]

use crate::{
    lexer::token::Token,
    utils::iter::{
        MorePeekableIter,
        MorePeekable
    }
};

pub struct Parser<'a, I: Iterator<Item = Token<'a>>> {
    iter: MorePeekableIter<I>,
}

impl<'a, I: Iterator<Item = Token<'a>>> Parser<'a, I> {
    pub fn new(iter: I) -> Parser<'a, I> {
        Parser { iter: iter.more_peekable() }
    }
}
