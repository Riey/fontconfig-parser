use crate::PropertyKind;
use serde::{Deserialize, Serialize};

/// This element contains a single value which is compared with the target ('pattern', 'font', 'scan' or 'default') property "property" (substitute any of the property names seen above).
/// 'compare' can be one of "eq", "not_eq", "less", "less_eq", "more", "more_eq", "contains" or "not_contains".
/// 'qual' may either be the default, "any", in which case the match succeeds if any value associated with the property matches the test value,
/// or "all", in which case all of the values associated with the property must match the test value. 'ignore-blanks' takes a boolean value.
/// if 'ignore-blanks' is set "true", any blanks in the string will be ignored on its comparison. this takes effects only when compare="eq" or compare="not_eq".
/// When used in a <match target="font"> element, the target= attribute in the <test> element selects between matching the original pattern or the font.
/// "default" selects whichever target the outer <match> element has selected.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Test {
    pub name: PropertyKind,
    #[serde(default)]
    pub qual: TestQual,
    #[serde(default)]
    pub target: TestTarget,
    #[serde(default)]
    pub compare: TestCompare,
    #[serde(rename = "string")]
    pub value: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestTarget {
    Default,
    Pattern,
    Font,
    Scan,
}

impl Default for TestTarget {
    fn default() -> Self {
        TestTarget::Default
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestCompare {
    Eq,
    NotEq,
    Less,
    LessEq,
    More,
    MoreEq,
    Contains,
    NotContains,
}

impl Default for TestCompare {
    fn default() -> Self {
        TestCompare::Eq
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestQual {
    Any,
    All,
}

impl Default for TestQual {
    fn default() -> Self {
        TestQual::Any
    }
}
