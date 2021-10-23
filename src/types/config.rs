#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Clone, Debug, Default)]
pub struct Config {
    // blank is no more used

    // pub blanks: Vec<Blank>,
    pub rescans: Vec<u32>,
}
