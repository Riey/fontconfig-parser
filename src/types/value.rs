use crate::{Constant, PropertyKind};
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

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
    Unary(UnaryOp, Vec<Self>),
    Binary(BinaryOp, Vec<Self>),
    Ternary(TernaryOp, Vec<Self>),
    List(ListOp, Vec<Self>),
    Matrix(Vec<Self>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyTarget {
    Default,
    Font,
    Pattern,
}

parse_enum! {
    PropertyTarget,
    (Default, "default"),
    (Font, "font"),
    (Pattern, "pattern"),
}

impl Default for PropertyTarget {
    fn default() -> Self {
        PropertyTarget::Default
    }
}

/// Runtime typed fontconfig value
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// <int>0</int>
    Int(Int),
    /// <double>1.5</double>
    Double(Double),
    /// <string>str</string>
    String(String),
    Constant(Constant),
    /// <bool>false</bool>
    Bool(Bool),
    Range(Int, Int),
    CharSet(CharSet),
    /// <name target="font">pixelsize</name>
    Property(PropertyTarget, PropertyKind),

    Prefer(Vec<String>),
    Accept(Vec<String>),
    Default(Vec<String>),
}

macro_rules! from_value {
	($($name:ident,)+) => {
        $(
            impl From<$name> for Value {
                fn from(v: $name) -> Value {
                    Value::$name(v)
                }
            }
        )+
	};
}

from_value! {
    Int,
    Bool,
    Double,
    String,
    Constant,
    CharSet,
}

impl From<(PropertyTarget, PropertyKind)> for Value {
    fn from((target, kind): (PropertyTarget, PropertyKind)) -> Self {
        Value::Property(target, kind)
    }
}

impl<V> From<V> for Expression
where
    Value: From<V>,
{
    fn from(v: V) -> Self {
        Expression::Simple(Value::from(v))
    }
}
