use serde::{Deserialize, Serialize};

pub type Bool = bool;
pub type Int = u32;
pub type Double = f64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharSet {
    #[serde(rename = "int")]
    pub ints: Vec<Int>,
}

/// Runtime typed fontconfig value
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Int { int: Int },
    Double { double: Double },
    String { string: String },
    Bool { bool: Bool },
    // Matrix
    // Range,
    CharSet { charset: CharSet },
}
