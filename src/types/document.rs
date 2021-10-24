use crate::{Alias, CacheDir, Config, Dir, Include, Match, SelectFont};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// <https://www.freedesktop.org/software/fontconfig/fontconfig-user.html>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Document {
    pub description: String,
    pub select_fonts: Vec<SelectFont>,
    pub dirs: Vec<Dir>,
    pub cache_dirs: Vec<CacheDir>,
    pub includes: Vec<Include>,
    pub matches: Vec<Match>,
    pub config: Config,
    pub aliases: Vec<Alias>,
}
