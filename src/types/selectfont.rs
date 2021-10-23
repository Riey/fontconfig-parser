use crate::Property;

#[derive(Clone, Debug, Default)]
pub struct SelectFont {
    pub rejects: Vec<FontMatch>,
    pub accepts: Vec<FontMatch>,
}

#[derive(Clone, Debug)]
pub enum FontMatch {
    Glob(String),
    Pattern(Vec<Property>),
}
