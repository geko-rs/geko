/// Imports
use crate::stmt::Block;
use std::fmt::Display;
use tick_lex::token::Span;

/// Assignment operator
#[derive(Debug, Clone, Copy)]
pub enum AssignOp {
    Assign, // =
    Add,    // +=
    Sub,    // -=
    Mul,    // *=
    Div,    // /=
    Mod,    // %=
    BitAnd, // &=
    BitOr,  // |=
    Xor,    // ^=
}

/// Binary operator
#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    Mod,    // %
    And,    // &&
    Or,     // ||
    Gt,     // >
    Ge,     // >=
    Lt,     // <
    Le,     // <=
    Eq,     // ==
    Ne,     // !=
    BitAnd, // &
    BitOr,  // |
    Xor,    // ^
}

/// Display implementation
impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Mod => write!(f, "%"),
            BinOp::And => write!(f, "&&"),
            BinOp::Or => write!(f, "||"),
            BinOp::Gt => write!(f, ">"),
            BinOp::Ge => write!(f, ">="),
            BinOp::Lt => write!(f, "<"),
            BinOp::Le => write!(f, "<="),
            BinOp::Eq => write!(f, "=="),
            BinOp::Ne => write!(f, "!="),
            BinOp::BitAnd => write!(f, "&"),
            BinOp::BitOr => write!(f, "|"),
            BinOp::Xor => write!(f, "^"),
        }
    }
}

/// Unary operator
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,  // -
    Bang, // !
}

/// Display implementation
impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Bang => write!(f, "!"),
        }
    }
}

/// Literal
#[derive(Debug, Clone)]
pub enum Lit {
    /// Number literal
    Number(String),
    /// String literal
    String(String),
    /// Bool literal
    Bool(String),
    /// Null literal
    Null,
}

/// Represents function
#[derive(Debug, Clone)]
pub struct Function {
    /// Function span
    pub span: Span,
    /// Function signature span
    pub sign_span: Span,
    /// Function name
    pub name: String,
    /// Function parameters
    pub params: Vec<String>,
    /// Function block
    pub block: Block,
}
