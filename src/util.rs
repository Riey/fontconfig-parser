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

macro_rules! eof {
    ($($tt:tt)*) => {
        return Err(crate::Error::Xml(quick_xml::Error::UnexpectedEof(format!($($tt)*))));
    };
}

macro_rules! parse_enum {
    (
        $ty:ty,
        $(
            ($variant:ident, $text:expr),
        )+
        |$arg:ident| $fallback:expr,
    ) => {
        impl std::str::FromStr for $ty {
            type Err = crate::Error;

            fn from_str($arg: &str) -> crate::Result<$ty> {
                match $arg {
                    $(
                        $text => Ok(<$ty>::$variant),
                    )+
                    _ => {
                        $fallback
                    }
                }
            }
        }
    };
    (
        $ty:ty,
        $(
            ($variant:ident, $text:expr),
        )+
    ) => {
        parse_enum! {
            $ty,
            $(
                ($variant, $text),
            )+
            |s| Err(crate::Error::ParseEnumError(std::any::type_name::<$ty>(), s.to_string())),
        }
    };
}
