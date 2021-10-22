use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    // blank is no more used

    // #[serde(rename = "blank")]
    // pub blanks: Vec<Blank>,
    #[serde(rename = "rescan")]
    pub rescans: Vec<Rescan>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Rescan {
    #[serde(rename = "int")]
    pub value: u32,
}
