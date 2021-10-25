use crate::{Int, IntOrRange};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Config {
    pub blanks: Vec<IntOrRange>,
    pub rescans: Vec<Int>,
}
