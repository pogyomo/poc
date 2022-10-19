#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stmt<'a> {
    Expr(Expr<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr<'a> {
    Integer(Integer),
    Identifier(Identifier<'a>),
    Infix(Infix<'a>),
}

impl<'a> Expr<'a> {
    pub fn upcast(self) -> Stmt<'a> {
        Stmt::Expr(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer {
    value: u64,
}

impl<'a> Integer {
    pub fn new(value: u64) -> Integer {
        Integer { value }
    }

    pub fn upcast(self) -> Expr<'a> {
        Expr::Integer(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier<'a> {
    name: &'a str,
}

impl<'a> Identifier<'a> {
    pub fn new(name: &'a str) -> Identifier<'a> {
        Identifier { name }
    }

    pub fn upcast(self) -> Expr<'a> {
        Expr::Identifier(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Infix<'a> {
    lhs: Box<Expr<'a>>,
    rhs: Box<Expr<'a>>,
    op: InfixOp,
}

impl<'a> Infix<'a> {
    pub fn new(lhs: Expr<'a>, rhs: Expr<'a>, op: InfixOp) -> Infix<'a> {
        Infix { lhs: lhs.into(), rhs: rhs.into(), op }
    }

    pub fn upcast(self) -> Expr<'a> {
        Expr::Infix(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InfixOp {
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
}
