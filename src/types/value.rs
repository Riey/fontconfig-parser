use crate::Constant;

pub type Bool = bool;
pub type Int = u32;
pub type Double = f64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharSet {
    pub ints: Vec<Int>,
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
    // Range,
    CharSet(CharSet),
}
