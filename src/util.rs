use std::io::BufRead;

use quick_xml::{events::attributes::Attribute, Reader};

use crate::Result;

pub trait AttributeExt {
    fn parse<T: std::str::FromStr, B: BufRead>(&self, reader: &Reader<B>) -> Result<T>
    where
        crate::Error: From<T::Err>;
}

impl<'a> AttributeExt for Attribute<'a> {
    fn parse<T: std::str::FromStr, B: BufRead>(&self, reader: &Reader<B>) -> Result<T>
    where
        crate::Error: From<T::Err>,
    {
        Ok(self.unescape_and_decode_without_bom(reader)?.parse()?)
    }
}
