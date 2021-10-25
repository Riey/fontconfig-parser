use crate::{Int, IntOrRange};

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Config {
    pub blanks: Vec<IntOrRange>,
    pub rescans: Vec<Int>,
}
