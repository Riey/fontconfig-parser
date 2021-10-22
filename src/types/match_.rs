mod edit;
mod test;

pub use self::edit::*;
pub use self::test::*;

#[derive(Clone, Debug, Default)]
pub struct Match {
    pub target: MatchTarget,
    pub tests: Vec<Test>,
    pub edits: Vec<Edit>,
}

#[derive(Clone, Copy, Debug)]
pub enum MatchTarget {
    Pattern,
    Font,
    Scan,
}

impl Default for MatchTarget {
    fn default() -> Self {
        MatchTarget::Pattern
    }
}
