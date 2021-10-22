use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Dir {
    #[serde(default)]
    pub prefix: DirPrefix,
    pub salt: Option<String>,
    #[serde(rename = "$value")]
    pub path: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CacheDir {
    #[serde(default)]
    pub prefix: DirPrefix,
    #[serde(rename = "$value")]
    pub path: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Include {
    #[serde(default)]
    pub prefix: DirPrefix,
    #[serde(with = "crate::util::serde_yesno")]
    #[serde(default)]
    pub ignore_missing: bool,
    #[serde(rename = "$value")]
    pub path: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirPrefix {
    Default,
    Cwd,
    Xdg,
}

impl Default for DirPrefix {
    fn default() -> Self {
        DirPrefix::Default
    }
}
