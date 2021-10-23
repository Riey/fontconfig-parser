use std::error;
use std::fmt;
use std::num::ParseFloatError;
use std::num::ParseIntError;
use std::str::ParseBoolError;

#[derive(Debug)]
pub enum Error {
    Xml(xmlparser::Error),
    UnexpectedEof(String),
    UnmatchedDocType,
    NoFontconfig,
    InvalidFormat,
    IoError(std::io::Error),
    ParseEnumError(&'static str, String),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    ParseBoolError(ParseBoolError),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<xmlparser::Error> for Error {
    fn from(e: xmlparser::Error) -> Self {
        Self::Xml(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloatError(e)
    }
}

impl From<ParseBoolError> for Error {
    fn from(e: ParseBoolError) -> Self {
        Self::ParseBoolError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Xml(e) => e.fmt(f),
            Error::UnexpectedEof(msg) => write!(f, "Get Unexpected eof: {}", msg),
            Error::UnmatchedDocType => write!(f, "DOCTYPE is not fontconfig"),
            Error::NoFontconfig => write!(f, "Can't find fontconfig element"),
            Error::InvalidFormat => write!(f, "Config format is invalid"),
            Error::IoError(e) => write!(f, "IO error: {}", e),
            Error::ParseEnumError(ty, s) => write!(f, "Unknown variant for {}: {}", ty, s),
            Error::ParseIntError(e) => e.fmt(f),
            Error::ParseFloatError(e) => e.fmt(f),
            Error::ParseBoolError(e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {}
