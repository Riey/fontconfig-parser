#[macro_use]
mod util;

mod error;
mod types;

use std::io::BufRead;
use util::AttributeExt;

pub type Result<T> = std::result::Result<T, Error>;

pub use crate::error::Error;
pub use crate::types::*;

pub use quick_xml;

use quick_xml::{events::Event, Reader};

/// https://www.freedesktop.org/software/fontconfig/fontconfig-user.html
#[derive(Clone, Debug, Default)]
pub struct Document {
    pub description: String,
    pub select_fonts: Vec<SelectFont>,
    pub dirs: Vec<Dir>,
    pub cache_dirs: Vec<CacheDir>,
    pub includes: Vec<Include>,
    pub matches: Vec<Match>,
    pub config: Config,
}

pub struct DocumentReader {
    buf: Vec<u8>,
}

impl DocumentReader {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(128),
        }
    }

    /// Clear internal buffer
    pub fn clear(&mut self) {
        self.buf.clear();
    }

    fn read_string<B: BufRead>(&mut self, tag: &[u8], reader: &mut Reader<B>) -> Result<String> {
        loop {
            match reader.read_event(&mut self.buf)? {
                Event::Start(s) => {
                    if s.name() == tag {
                        break Ok(reader.read_text(tag, &mut self.buf)?);
                    } else {
                        break Err(Error::InvalidFormat);
                    }
                }
                Event::Eof => eof!("Expect {:?}", tag),
                _ => {}
            }
        }
    }

    fn read_value<B: BufRead>(&mut self, reader: &mut Reader<B>) -> Result<Expression> {
        loop {
            match reader.read_event(&mut self.buf)? {
                Event::End(_) => break Err(Error::InvalidFormat),
                Event::Start(s) => match s.name() {
                    b"string" => {
                        break Ok(Value::String(reader.read_text(b"string", &mut self.buf)?).into());
                    }
                    b"double" => {
                        break Ok(Value::Double(
                            reader.read_text(b"double", &mut self.buf)?.parse()?,
                        )
                        .into());
                    }
                    b"int" => {
                        break Ok(
                            Value::Int(reader.read_text(b"int", &mut self.buf)?.parse()?).into(),
                        );
                    }
                    b"bool" => {
                        break Ok(
                            Value::Bool(reader.read_text(b"bool", &mut self.buf)?.parse()?).into(),
                        );
                    }
                    b"const" => {
                        break Ok(Value::Const(
                            reader.read_text(b"const", &mut self.buf)?.parse()?,
                        )
                        .into());
                    }
                    b"matrix" => {
                        let ret = Ok(Value::Matrix([
                            self.read_string(b"double", reader)?.parse()?,
                            self.read_string(b"double", reader)?.parse()?,
                            self.read_string(b"double", reader)?.parse()?,
                            self.read_string(b"double", reader)?.parse()?,
                        ])
                        .into());

                        reader.read_to_end(b"matrix", &mut self.buf)?;

                        break ret;
                    }
                    b"name" => {
                        break Ok(Value::Property(
                            reader.read_text(b"name", &mut self.buf)?.parse()?,
                        )
                        .into());
                    }
                    name => {
                        let name = std::str::from_utf8(name)
                            .map_err(quick_xml::Error::from)?
                            .to_string();

                        break if let Ok(list_op) = name.parse() {
                            let mut list = Vec::new();
                            while let Ok(value) = self.read_value(reader) {
                                list.push(value);
                            }
                            Ok(Expression::List(list, list_op))
                        } else if let Ok(unary_op) = name.parse() {
                            Ok(Expression::Unary(
                                Box::new(self.read_value(reader)?),
                                unary_op,
                            ))
                        } else if let Ok(binary_op) = name.parse() {
                            break Ok(Expression::Binary(
                                Box::new(self.read_value(reader)?),
                                Box::new(self.read_value(reader)?),
                                binary_op,
                            ));
                        } else if let Ok(ternary_op) = name.parse() {
                            break Ok(Expression::Ternary(
                                Box::new(self.read_value(reader)?),
                                Box::new(self.read_value(reader)?),
                                Box::new(self.read_value(reader)?),
                                ternary_op,
                            ));
                        } else {
                            todo!("{:?}", name)
                        };
                    }
                },
                Event::Eof => eof!("Expect property"),
                _ => {}
            }
        }
    }

    fn read_match<B: BufRead>(&mut self, reader: &mut Reader<B>) -> Result<Match> {
        let mut ret = Match::default();

        loop {
            match reader.read_event(&mut self.buf)? {
                Event::Text(_) | Event::Comment(_) => continue,
                Event::Start(s) => match s.name() {
                    b"test" => {
                        let mut test = Test::default();
                        let mut name = PropertyKind::default();

                        for attr in s.attributes() {
                            let attr = attr?;
                            match attr.key {
                                b"name" => name = attr.parse(reader)?,
                                b"qual" => test.qual = attr.parse(reader)?,
                                b"target" => test.target = attr.parse(reader)?,
                                b"compare" => test.compare = attr.parse(reader)?,
                                _ => {}
                            }
                        }

                        test.value = name.make_property(self.read_value(reader)?);
                        reader.read_to_end(b"test", &mut self.buf)?;

                        ret.tests.push(test);
                    }
                    b"edit" => {
                        let mut edit = Edit::default();
                        let mut name = PropertyKind::default();

                        for attr in s.attributes() {
                            let attr = attr?;

                            match attr.key {
                                b"name" => {
                                    name = attr.parse(reader).map_err(|e| {
                                        eprintln!("{:?}", attr);
                                        e
                                    })?
                                }
                                b"mode" => edit.mode = attr.parse(reader)?,
                                b"binding" => edit.binding = attr.parse(reader)?,
                                _ => {}
                            }
                        }

                        edit.value = name.make_property(self.read_value(reader)?);
                        reader.read_to_end(b"edit", &mut self.buf)?;

                        ret.edits.push(edit);
                    }
                    _ => {}
                },
                Event::End(e) => {
                    if e.name() == b"match" {
                        break;
                    }
                }
                Event::Eof => break,
                _ => {}
            }
        }

        Ok(ret)
    }

    fn read_pattern<B: BufRead>(&mut self, reader: &mut Reader<B>) -> Result<FontMatch> {
        let mut patterns = Vec::new();

        loop {
            match reader.read_event(&mut self.buf)? {
                Event::Start(s) => match s.name() {
                    b"patelt" => {
                        let kind = match s.attributes().find_map(|a| match a {
                            Ok(a) => {
                                if a.key == b"name" {
                                    Some(a.parse(reader))
                                } else {
                                    None
                                }
                            }
                            Err(err) => Some(Err(err.into())),
                        }) {
                            Some(kind) => kind?,
                            None => PropertyKind::default(),
                        };

                        let value = self.read_value(reader)?;

                        patterns.push(kind.make_property(value));
                    }
                    _ => return Err(Error::InvalidFormat),
                },
                Event::End(e) if e.name() == b"pattern" => break,
                Event::Eof => eof!("Expected pattern"),
                _ => {}
            }
        }

        Ok(FontMatch::Pattern(patterns))
    }

    fn read_glob<B: BufRead>(&mut self, reader: &mut Reader<B>) -> Result<FontMatch> {
        let pat = reader.read_text(b"glob", &mut self.buf)?;
        Ok(FontMatch::Glob(pat))
    }

    fn read_selectfont<B: BufRead>(&mut self, reader: &mut Reader<B>) -> Result<SelectFont> {
        let mut ret = SelectFont::default();

        loop {
            match reader.read_event(&mut self.buf)? {
                Event::Start(s) => match s.name() {
                    b"acceptfont" => loop {
                        match reader.read_event(&mut self.buf)? {
                            Event::Start(s) => match s.name() {
                                b"pattern" => ret.accepts.push(self.read_pattern(reader)?),
                                b"glob" => ret.accepts.push(self.read_glob(reader)?),
                                _ => {}
                            },
                            Event::End(e) if e.name() == b"acceptfont" => break,
                            Event::Eof => eof!("Expected fontmatch"),
                            _ => {}
                        }
                    },
                    b"rejectfont" => loop {
                        match reader.read_event(&mut self.buf)? {
                            Event::Start(s) => match s.name() {
                                b"pattern" => ret.rejects.push(self.read_pattern(reader)?),
                                b"glob" => ret.rejects.push(self.read_glob(reader)?),
                                _ => {}
                            },
                            Event::End(e) if e.name() == b"rejectfont" => break,
                            Event::Eof => eof!("Expected fontmatch"),
                            _ => {}
                        }
                    },
                    _ => {}
                },
                Event::End(e) => {
                    if e.name() == b"selectfont" {
                        break Ok(ret);
                    }
                }
                Event::Eof => {
                    eof!("Expected selectfont");
                }
                _ => {}
            }
        }
    }

    fn read_config<B: BufRead>(&mut self, reader: &mut Reader<B>) -> Result<Config> {
        let mut ret = Config::default();

        loop {
            match reader.read_event(&mut self.buf)? {
                Event::Start(s) => match s.name() {
                    b"rescan" => {
                        let n = self.read_string(b"int", reader)?.parse()?;
                        ret.rescans.push(n);
                    }
                    _ => {}
                },
                Event::End(e) => {
                    if e.name() == b"config" {
                        break Ok(ret);
                    }
                }
                Event::Eof => eof!("Expected config"),
                _ => {}
            }
        }
    }

    /// Read `Document` from `quick_xml::Reader`
    pub fn read_document<B: BufRead>(&mut self, reader: &mut Reader<B>) -> Result<Document> {
        self.clear();

        // STAGE 1. validate document

        loop {
            match reader.read_event(&mut self.buf)? {
                Event::Decl(_) | Event::Text(_) | Event::Comment(_) => continue,
                Event::DocType(doc_type) => match doc_type.as_ref() {
                    b" fontconfig SYSTEM \"urn:fontconfig:fonts.dtd\""
                    | b" fontconfig SYSTEM 'urn:fontconfig:fonts.dtd'" => {}
                    _ => return Err(Error::UnmatchedDocType),
                },
                Event::Start(s) => {
                    if s.name() == b"fontconfig" {
                        break;
                    }
                }
                _ => return Err(Error::NoFontconfig),
            }
        }

        let mut ret = Document::default();

        // STAGE 2. read elements

        loop {
            match reader.read_event(&mut self.buf)? {
                Event::Start(s) => match s.name() {
                    b"selectfont" => {
                        ret.select_fonts.push(self.read_selectfont(reader)?);
                    }
                    b"alias" => {
                        // TODO
                        reader.read_to_end(b"alias", &mut self.buf)?;
                    }
                    b"description" => {
                        ret.description = reader.read_text(b"description", &mut self.buf)?;
                    }
                    b"match" => {
                        ret.matches.push(self.read_match(reader)?);
                    }
                    b"config" => {
                        ret.config = self.read_config(reader)?;
                    }
                    b"dir" => {
                        let mut dir = Dir::default();

                        for attr in s.attributes() {
                            let attr = attr?;

                            match attr.key {
                                b"prefix" => {
                                    dir.prefix = attr.parse(reader)?;
                                }
                                b"salt" => {
                                    dir.salt = Some(attr.unescape_and_decode_value(reader)?);
                                }
                                _ => {}
                            }
                        }

                        dir.path = reader.read_text(b"dir", &mut self.buf)?;

                        ret.dirs.push(dir);
                    }
                    b"cachedir" => {
                        let mut dir = CacheDir::default();

                        for attr in s.attributes() {
                            let attr = attr?;

                            match attr.key {
                                b"prefix" => {
                                    dir.prefix = attr.parse(reader)?;
                                }
                                _ => {}
                            }
                        }

                        dir.path = reader.read_text(b"cachedir", &mut self.buf)?;

                        ret.cache_dirs.push(dir);
                    }
                    b"include" => {
                        let mut dir = Include::default();

                        for attr in s.attributes() {
                            let attr = attr?;

                            match attr.key {
                                b"prefix" => {
                                    dir.prefix = attr.parse(reader)?;
                                }
                                b"ignore_missing" => match attr.unescaped_value()?.as_ref() {
                                    b"yes" => {
                                        dir.ignore_missing = true;
                                    }
                                    b"no" => {
                                        dir.ignore_missing = false;
                                    }
                                    _ => {
                                        return Err(Error::InvalidFormat);
                                    }
                                },
                                _ => {}
                            }
                        }

                        dir.path = reader.read_text(b"include", &mut self.buf)?;

                        ret.includes.push(dir);
                    }
                    _ => {
                        eprintln!("Unknown element: {:?}", s);
                    }
                },
                Event::Eof => break,
                _ => {}
            }
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let mut doc_reader = DocumentReader::new();
        let doc = doc_reader
            .read_document(&mut quick_xml::Reader::from_str(include_str!(
                "/etc/fonts/fonts.conf"
            )))
            .unwrap();

        dbg!(doc);
    }
}
