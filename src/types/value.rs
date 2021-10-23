use crate::{Constant, PropertyKind};

pub type Bool = bool;
pub type Int = u32;
pub type Double = f64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharSet {
    pub ints: Vec<Int>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ListOp {
    Times,
    Divide,
    Or,
    And,
    Plus,
    Minus,
}

parse_enum! {
    ListOp,
    (Times, "times"),
    (Divide, "divide"),
    (Or, "or"),
    (And, "and"),
    (Plus, "plus"),
    (Minus, "minus"),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Not,
}

parse_enum! {
    UnaryOp,
    (Not, "not"),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Eq,
    NotEq,
    Less,
    LessEq,
    More,
    MoreEq,
    Contains,
    NotContains,
}

parse_enum! {
    BinaryOp,
    (Eq, "eq"),
    (NotEq, "noteq"),
    (Less, "less"),
    (LessEq, "lesseq"),
    (More, "more"),
    (MoreEq, "moreeq"),
    (Contains, "contains"),
    (NotContains, "notcontains"),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TernaryOp {
    If,
}

parse_enum! {
    TernaryOp,
    (If, "if"),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Simple(Value),
    Unary(Box<Self>, UnaryOp),
    Binary(Box<Self>, Box<Self>, BinaryOp),
    Ternary(Box<Self>, Box<Self>, Box<Self>, TernaryOp),
    List(Vec<Self>, ListOp),
}

impl From<Value> for Expression {
    fn from(v: Value) -> Self {
        Expression::Simple(v)
    }
}

/// Runtime typed fontconfig value
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(Int),
    Double(Double),
    String(String),
    Const(Constant),
    Bool(Bool),
    Matrix([Double; 4]),
    Range(Int, Int),
    CharSet(CharSet),
    Property(PropertyKind),

    Prefer(Vec<String>),
    Accept(Vec<String>),
    Default(Vec<String>),
}
