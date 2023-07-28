use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    UnaryMinus(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    Function {
        name: String,
        args: Vec<Box<Expr>>,
    },
    Symbol(i32),
    Literal(i32),
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::BinOp { lhs, op, rhs } => write!(f, "BinOp {{ lhs: Box::new({lhs}), op: {op:?}, rhs: Box::new({rhs})}}"),
            Expr::UnaryMinus(expr) => write!(f, "UnaryMinus(Box::new({expr}))"),
            _ => write!(f, "{self:?}"),
        }
    }
}
