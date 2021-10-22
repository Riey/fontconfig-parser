pub mod serde_yesno {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &bool, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(if *v { "yes" } else { "no" })
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
        let s = String::deserialize(d)?;

        match s.as_str() {
            "yes" => Ok(true),
            "no" => Ok(false),
            other => Err(Error::unknown_variant(other, &["yes", "no"])),
        }
    }
}
