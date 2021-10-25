use crate::Property;

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
