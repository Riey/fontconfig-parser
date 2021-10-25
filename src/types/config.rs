use crate::{Int, IntOrRange};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Config {
    pub blanks: Vec<IntOrRange>,
    pub rescans: Vec<Int>,
}
