#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Dir {
    pub prefix: DirPrefix,
    pub salt: String,
    pub path: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CacheDir {
    pub prefix: DirPrefix,
    pub path: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Include {
    pub prefix: DirPrefix,
    pub ignore_missing: bool,
    pub path: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RemapDir {
    pub prefix: DirPrefix,
    pub as_path: String,
    pub salt: String,
    pub path: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirPrefix {
    Default,
    Cwd,
    Xdg,
    Relative,
}

parse_enum! {
    DirPrefix,
    (Default, "default"),
    (Cwd, "cwd"),
    (Xdg, "xdg"),
    (Relative, "relative"),
}

impl Default for DirPrefix {
    fn default() -> Self {
        DirPrefix::Default
    }
}

macro_rules! define_calculate_path {
    ($ty:ident, $xdg_env:expr, $xdg_fallback:expr) => {
        impl $ty {
            /// Environment variable name which used `xdg` prefix
            pub const XDG_ENV: &'static str = $xdg_env;
            /// Fallback path when `XDG_ENV` is not exists
            pub const XDG_FALLBACK_PATH: &'static str = $xdg_fallback;

            /// Calculate actual path
            pub fn calculate_path<P: AsRef<std::path::Path> + ?Sized>(
                &self,
                config_file_path: &P,
            ) -> std::path::PathBuf {
                match self.prefix {
                    DirPrefix::Default => self.path.clone().into(),
                    DirPrefix::Cwd => std::path::Path::new(".").join(&self.path),
                    DirPrefix::Relative => config_file_path.as_ref().join(&self.path),
                    DirPrefix::Xdg => std::path::PathBuf::from(
                        std::env::var($xdg_env).unwrap_or_else(|_| $xdg_fallback.into()),
                    )
                    .join(&self.path),
                }
            }
        }
    };
}

define_calculate_path!(Dir, "XDG_DATA_HOME", "~/.local/share");
define_calculate_path!(CacheDir, "XDG_CACHE_HOME", "~/.cache");
define_calculate_path!(Include, "XDG_CONFIG_HOME", "~/.config");
