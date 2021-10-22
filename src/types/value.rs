pub type Bool = bool;
pub type Int = u32;
pub type Double = f64;

#[derive(Clone, Debug)]
pub struct CharSet {
    pub ints: Vec<Int>,
}

/// Runtime typed fontconfig value
#[derive(Clone, Debug)]
pub enum Value {
    Int(Int),
    Double(Double),
    String(String),
    Bool(Bool),
    // Matrix
    // Range,
    CharSet(CharSet),
}
