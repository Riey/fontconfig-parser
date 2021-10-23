//! This crate provide parsing fontconfig file but not yet complete all features
//!
//! see <https://www.freedesktop.org/software/fontconfig/fontconfig-user.html> for more detail infomation of fontconfig file
//!
//! # Example
//! 
//! ```rust
//! if let Ok(document_str) = std::fs::read_to_string("/etc/fonts/fonts.conf") {
//!     let document = fontconfig_parser::parse_document_from_str(&document_str).unwrap();
//! }
//! ```

#[macro_use]
mod util;

mod error;
mod parser;
mod types;

pub type Result<T> = std::result::Result<T, Error>;

pub use crate::error::Error;
pub use crate::types::*;

pub fn parse_document_from_str(s: &str) -> Result<Document> {
    crate::parser::parse_document(
        &mut xmlparser::Tokenizer::from(s)
            .into_iter()
            .map(|r| r.map_err(Into::into)),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let doc = crate::parse_document_from_str(include_str!(
            "../test-conf/conf.d/10-scale-bitmap-fonts.conf"
        ))
        .unwrap();

        dbg!(doc);
    }
}
