use crate::Property;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use alloc::string::String;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectFont {
    pub rejects: Vec<FontMatch>,
    pub accepts: Vec<FontMatch>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FontMatch {
    Glob(String),
    Pattern(Vec<Property>),
}
