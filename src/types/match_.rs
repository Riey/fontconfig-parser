mod edit;
mod test;

pub use self::edit::*;
pub use self::test::*;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Clone, Debug, Default)]
pub struct Match<'a> {
    pub target: MatchTarget,
    pub tests: Vec<Test<'a>>,
    pub edits: Vec<Edit<'a>>,
}

#[derive(Clone, Copy, Debug)]
pub enum MatchTarget {
    Pattern,
    Font,
    Scan,
}

parse_enum! {
    MatchTarget,
    (Pattern, "pattern"),
    (Font, "font"),
    (Scan, "scan"),
}

impl Default for MatchTarget {
    fn default() -> Self {
        MatchTarget::Pattern
    }
}
