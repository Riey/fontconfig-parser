mod edit;
mod test;

pub use self::edit::*;
pub use self::test::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Match {
    #[serde(default)]
    target: MatchTarget,
    #[serde(rename = "test")]
    tests: Vec<Test>,
    #[serde(rename = "edit")]
    edits: Vec<Edit>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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
