//! This crate provide parsing fontconfig file but not yet complete all features
//!
//! see <https://www.freedesktop.org/software/fontconfig/fontconfig-user.html> for more detail infomation of fontconfig file
//!
//! # Example
//!
//! ```rust
//! use fontconfig_parser::parse_document_from_str;
//!
//! if let Ok(document_str) = std::fs::read_to_string("/etc/fonts/fonts.conf") {
//!     let document = parse_document_from_str(&document_str).unwrap();
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[macro_use]
mod util;

mod error;
mod parser;
mod types;

pub type Result<T> = core::result::Result<T, Error>;

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
        crate::parse_document_from_str(include_str!(
            "../test-conf/conf.d/10-scale-bitmap-fonts.conf"
        ))
        .unwrap();
    }
}
