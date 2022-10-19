#![allow(dead_code)]

use anyhow::{anyhow, bail};

use crate::{
    lexer::token::{
        Token,
        TokenKind
    },
    utils::iter::{
        MorePeekableIter,
        MorePeekable
    }
};
use self::ast::{
    Stmt,
    Expr, Identifier, Infix, InfixOp,
};

pub mod ast;

pub struct Parser<'a, I: Iterator<Item = Token<'a>>> {
    iter: MorePeekableIter<I>,
}

impl<'a, I: Iterator<Item = Token<'a>>> Parser<'a, I> {
    pub fn new(iter: I) -> Parser<'a, I> {
        Parser { iter: iter.more_peekable() }
    }
}

impl<'a, I: Iterator<Item = Token<'a>>> Parser<'a, I> {
    pub fn parse_stmt(&mut self) -> anyhow::Result<Stmt<'a>> {
        self.parse_stmt_expr()
    }

    fn parse_stmt_expr(&mut self) -> anyhow::Result<Stmt<'a>> {
        let expr = self.parse_expr()?;
        if self.peek_curr()?.kind == TokenKind::Semicolon {
            self.next();
            Ok(expr.upcast())
        } else {
            bail!("Expression statement must be finished with ';'")
        }
    }
}

impl<'a, I: Iterator<Item = Token<'a>>> Parser<'a, I> {
    fn parse_expr(&mut self) -> anyhow::Result<Expr<'a>> {
        self.parse_expr_addition()
    }

    fn parse_expr_addition(&mut self) -> anyhow::Result<Expr<'a>> {
        let mut lhs = self.parse_expr_multiplication()?;
        while let Ok(token) = self.peek_curr() {
            match token.kind {
                TokenKind::Plus => {
                    self.next();
                    lhs = Infix::new(lhs, self.parse_expr_multiplication()?, InfixOp::Add).upcast();
                }
                TokenKind::Minus => {
                    self.next();
                    lhs = Infix::new(lhs, self.parse_expr_multiplication()?, InfixOp::Sub).upcast();
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_expr_multiplication(&mut self) -> anyhow::Result<Expr<'a>> {
        let mut lhs = self.parse_expr_primary()?;
        while let Ok(token) = self.peek_curr() {
            match token.kind {
                TokenKind::Star => {
                    self.next();
                    lhs = Infix::new(lhs, self.parse_expr_primary()?, InfixOp::Mul).upcast();
                }
                TokenKind::Slash => {
                    self.next();
                    lhs = Infix::new(lhs, self.parse_expr_primary()?, InfixOp::Div).upcast();
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_expr_primary(&mut self) -> anyhow::Result<Expr<'a>> {
        match self.peek_curr()?.kind {
            TokenKind::Identifier => Ok(self.parse_expr_identifier()?.upcast()),
            kind => bail!("{:?} can't be a primary.", kind),
        }
    }

    fn parse_expr_identifier(&mut self) -> anyhow::Result<Identifier<'a>> {
        let name = self.peek_curr()?.span.as_str();
        self.next();
        Ok(Identifier::new(name))
    }
}

impl<'a, I: Iterator<Item = Token<'a>>> Parser<'a, I> {
    fn peek_curr(&self) -> anyhow::Result<&Token<'a>> {
        self.iter.peek_curr().ok_or(anyhow!("Failed to get current token"))
    }

    fn peek_next(&self) -> anyhow::Result<&Token<'a>> {
        self.iter.peek_next().ok_or(anyhow!("Failed to get next token"))
    }

    fn next(&mut self) {
        self.iter.next();
    }
}
