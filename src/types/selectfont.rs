use crate::Property;

#[derive(Clone, Debug, Default)]
pub struct SelectFont<'a> {
    pub rejects: Vec<FontMatch<'a>>,
    pub accepts: Vec<FontMatch<'a>>,
}

#[derive(Clone, Debug)]
pub enum FontMatch<'a> {
    Glob(&'a str),
    Pattern(Vec<Property<'a>>),
}
