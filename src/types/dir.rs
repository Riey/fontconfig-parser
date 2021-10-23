use std::env;
use std::path::{Path, PathBuf};

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

#[derive(Clone, Copy, Debug)]
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
    ($ty:ty, $xdg_env:expr, $xdg_fallback:expr) => {
        impl $ty {
            /// Calculate actual path
            pub fn calculate_path<P: AsRef<Path> + ?Sized>(&self, config_file_path: &P) -> PathBuf {
                match self.prefix {
                    DirPrefix::Default => self.path.as_str().into(),
                    DirPrefix::Cwd => Path::new(".").join(&self.path),
                    DirPrefix::Relative => config_file_path.as_ref().join(&self.path),
                    DirPrefix::Xdg => {
                        PathBuf::from(env::var($xdg_env).unwrap_or_else(|_| $xdg_fallback.into()))
                            .join(&self.path)
                    }
                }
            }
        }
    };
}

define_calculate_path!(Dir, "XDG_DATA_HOME", "~/.local/share");
define_calculate_path!(CacheDir, "XDG_CACHE_HOME", "~/.cache");
define_calculate_path!(Include, "XDG_CONFIG_HOME", "~/.config");
