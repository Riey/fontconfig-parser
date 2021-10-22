mod types;
mod util;

pub use crate::types::*;
use serde::{Deserialize, Serialize};

/// https://www.freedesktop.org/software/fontconfig/fontconfig-user.html
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Document {
    pub description: String,
    #[serde(rename = "dir")]
    pub dirs: Vec<Dir>,
    #[serde(rename = "cachedir")]
    pub cache_dirs: Vec<CacheDir>,
    #[serde(rename = "include")]
    pub includes: Vec<Include>,
    #[serde(rename = "match")]
    pub matches: Vec<Match>,
    #[serde(default)]
    pub config: Config,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let doc: crate::Document =
            quick_xml::de::from_str(include_str!("/etc/fonts/fonts.conf")).unwrap();

        assert_eq!(doc.description, "Default configuration file");
        dbg!(doc);
    }
}
