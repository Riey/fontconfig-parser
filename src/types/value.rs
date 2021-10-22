use crate::{Constant, PropertyKind};
use strum_macros::EnumString;

pub type Bool = bool;
pub type Int = u32;
pub type Double = f64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharSet {
    pub ints: Vec<Int>,
}

#[derive(Copy, Clone, Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum ListOp {
    Times,
    Divide,
    Or,
    And,
    Plus,
    Minus,
}

#[derive(Copy, Clone, Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
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

#[derive(Copy, Clone, Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum UnaryOp {
    Not,
}

#[derive(Copy, Clone, Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum TernaryOp {
    Conditional,
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
