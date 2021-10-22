use strum_macros::EnumString;

#[derive(Clone, Debug, Default)]
pub struct Dir {
    pub prefix: DirPrefix,
    pub salt: Option<String>,
    pub path: String,
}

#[derive(Clone, Debug, Default)]
pub struct CacheDir {
    pub prefix: DirPrefix,
    pub path: String,
}

#[derive(Clone, Debug, Default)]
pub struct Include {
    pub prefix: DirPrefix,
    pub ignore_missing: bool,
    pub path: String,
}

#[derive(Clone, Copy, Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
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
