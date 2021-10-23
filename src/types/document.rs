use crate::{CacheDir, Config, Dir, Include, Match, SelectFont};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// https://www.freedesktop.org/software/fontconfig/fontconfig-user.html
#[derive(Clone, Debug, Default)]
pub struct Document<'a> {
    pub description: &'a str,
    pub select_fonts: Vec<SelectFont<'a>>,
    pub dirs: Vec<Dir<'a>>,
    pub cache_dirs: Vec<CacheDir<'a>>,
    pub includes: Vec<Include<'a>>,
    pub matches: Vec<Match<'a>>,
    pub config: Config,
}
