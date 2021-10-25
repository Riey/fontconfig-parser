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
    crate::parser::parse_document(&roxmltree::Document::parse(s)?)
}

#[cfg(test)]
mod tests {}
