use crate::*;
use xmlparser::{ElementEnd, ExternalId, Token};

pub fn parse_document<'a, 'b>(
    tokens: &mut impl Iterator<Item = Result<Token<'a>>>,
) -> Result<Document<'a>> {
    // STAGE 1. validate document

    loop {
        match try_opt!(tokens.next(), "Expect fontconfig")? {
            Token::Declaration { .. } | Token::Text { .. } | Token::Comment { ..} => continue,
            Token::EmptyDtd { name, external_id, .. } => {
                if name.as_str() != "fontconfig" || external_id.map_or(false, |id| matches!(id, ExternalId::System(s) if s.as_str() != "urn:fontconfig:fonts.dtd")) {
return Err(Error::UnmatchedDocType);
                }
            }
            Token::ElementStart { local, .. } => {
                if local.as_str() == "fontconfig" {
                    break;
                }
            }
            _ => return Err(Error::NoFontconfig),
        }
    }

    // STAGE 2. read elements

    let mut doc = Document::default();

    loop {
        let token = match tokens.next() {
            Some(t) => t?,
            None => break,
        };

        match token {
            Token::ElementStart { local, .. } => match local.as_str() {
                "description" => doc.description = parse_string(local.as_str(), tokens)?,
                "config" => doc.config = parse_config(tokens)?,
                "selectfont" => doc.select_fonts.push(parse_selectfont(tokens)?),
                "dir" => {
                    let mut dir = Dir::default();

                    parse_attrs!(tokens, {
                        "prefix" => dir.prefix,
                    }, {
                        "salt" => dir.salt,
                    });

                    dir.path = parse_string("dir", tokens)?;

                    doc.dirs.push(dir);
                }
                "include" => {
                    let mut dir = Include::default();

                    parse_attrs!(tokens, {
                        "prefix" => dir.prefix,
                    });

                    dir.path = parse_string("include", tokens)?;

                    doc.includes.push(dir);
                }
                "cachedir" => {
                    let mut dir = CacheDir::default();

                    parse_attrs!(tokens, {
                        "prefix" => dir.prefix,
                    });

                    dir.path = parse_string("cachedir", tokens)?;

                    doc.cache_dirs.push(dir);
                }
                "match" => {
                    doc.matches.push(parse_match(tokens)?);
                }
                other => eprintln!("Unknown element: {}", other),
            },
            _ => {}
        }
    }

    Ok(doc)
}

fn parse_string<'a>(
    tag: &str,
    tokens: &mut impl Iterator<Item = Result<Token<'a>>>,
) -> Result<&'a str> {
    let mut ret = "";

    loop {
        match try_opt!(tokens.next(), "Expect: {}", tag)? {
            Token::ElementEnd {
                end: ElementEnd::Empty,
                ..
            } => break,
            Token::ElementEnd {
                end: ElementEnd::Close(_, e),
                ..
            } if e.as_str() == tag => break,
            Token::Text { text } => ret = text.as_str(),
            _ => {}
        }
    }

    Ok(ret)
}

fn parse_expr<'a>(tokens: &mut impl Iterator<Item = Result<Token<'a>>>) -> Result<Expression<'a>> {
    loop {
        match try_opt!(tokens.next(), "Expect expression")? {
            Token::ElementStart { local, .. } => match local.as_str() {
                "string" => return Ok(Value::String(parse_string("string", tokens)?).into()),
                "double" => {
                    return Ok(Value::Double(parse_string("double", tokens)?.parse()?).into())
                }
                "int" => return Ok(Value::Int(parse_string("int", tokens)?.parse()?).into()),
                "bool" => return Ok(Value::Bool(parse_string("bool", tokens)?.parse()?).into()),
                "const" => return Ok(Value::Const(parse_string("const", tokens)?.parse()?).into()),
                "matrix" => {
                    return Ok(Value::Matrix([
                        parse_string("double", tokens)?.parse()?,
                        parse_string("double", tokens)?.parse()?,
                        parse_string("double", tokens)?.parse()?,
                        parse_string("double", tokens)?.parse()?,
                    ])
                    .into())
                }
                "name" => return Ok(Value::Property(parse_string("name", tokens)?.parse()?).into()),

                name => {
                    return if let Ok(list_op) = name.parse() {
                        let mut list = Vec::new();
                        while let Ok(value) = parse_expr(tokens) {
                            list.push(value);
                        }
                        Ok(Expression::List(list, list_op))
                    } else if let Ok(unary_op) = name.parse() {
                        Ok(Expression::Unary(Box::new(parse_expr(tokens)?), unary_op))
                    } else if let Ok(binary_op) = name.parse() {
                        break Ok(Expression::Binary(
                            Box::new(parse_expr(tokens)?),
                            Box::new(parse_expr(tokens)?),
                            binary_op,
                        ));
                    } else if let Ok(ternary_op) = name.parse() {
                        break Ok(Expression::Ternary(
                            Box::new(parse_expr(tokens)?),
                            Box::new(parse_expr(tokens)?),
                            Box::new(parse_expr(tokens)?),
                            ternary_op,
                        ));
                    } else {
                        todo!("{:?}", local)
                    };
                }
            },
            Token::ElementEnd { .. } => return Err(Error::InvalidFormat),
            _ => {}
        }
    }
}

fn parse_selectfont<'a>(
    tokens: &mut impl Iterator<Item = Result<Token<'a>>>,
) -> Result<SelectFont<'a>> {
    let mut s = SelectFont::default();

    loop {
        match try_opt!(tokens.next(), "Expect selectfont")? {
            Token::ElementStart { local, .. } => match local.as_str() {
                "acceptfont" => loop {
                    match try_opt!(tokens.next(), "Expect acceptfont")? {
                        Token::ElementStart { local, .. } => match local.as_str() {
                            "glob" => s.accepts.push(parse_glob(tokens)?),
                            "pattern" => s.accepts.push(parse_pattern(tokens)?),
                            _ => {}
                        },
                        Token::ElementEnd {
                            end: ElementEnd::Close(_, e),
                            ..
                        } if e.as_str() == "acceptfont" => break,
                        _ => {}
                    }
                },
                "rejectfont" => loop {
                    match try_opt!(tokens.next(), "Expect rejectfont")? {
                        Token::ElementStart { local, .. } => match local.as_str() {
                            "glob" => s.rejects.push(parse_glob(tokens)?),
                            "pattern" => s.rejects.push(parse_pattern(tokens)?),
                            _ => {}
                        },
                        Token::ElementEnd {
                            end: ElementEnd::Close(_, e),
                            ..
                        } if e.as_str() == "rejectfont" => break,
                        _ => {}
                    }
                },
                _ => {}
            },
            Token::ElementEnd {
                end: ElementEnd::Close(_, e),
                ..
            } if e.as_str() == "selectfont" => break,
            _ => {}
        }
    }

    Ok(s)
}

fn parse_glob<'a>(tokens: &mut impl Iterator<Item = Result<Token<'a>>>) -> Result<FontMatch<'a>> {
    Ok(FontMatch::Glob(parse_string("glob", tokens)?))
}

fn parse_pattern<'a>(
    tokens: &mut impl Iterator<Item = Result<Token<'a>>>,
) -> Result<FontMatch<'a>> {
    let mut patterns = Vec::new();

    loop {
        match try_opt!(tokens.next(), "Expect pattern")? {
            Token::ElementStart { local, .. } => match local.as_str() {
                "patelt" => {
                    let mut kind = PropertyKind::default();

                    parse_attrs!(tokens, {
                        "name" => kind,
                    });

                    patterns.push(kind.make_property(parse_expr(tokens)?));
                }
                _ => {}
            },
            Token::ElementEnd {
                end: ElementEnd::Close(_, e),
                ..
            } if e.as_str() == "pattern" => break,
            _ => {}
        }
    }

    Ok(FontMatch::Pattern(patterns))
}

fn parse_match<'a>(tokens: &mut impl Iterator<Item = Result<Token<'a>>>) -> Result<Match<'a>> {
    let mut m = Match::default();

    parse_attrs!(tokens, {
        "target" => m.target,
    });

    loop {
        match try_opt!(tokens.next(), "Expect test or edit")? {
            Token::ElementStart { local, .. } => match local.as_str() {
                "test" => {
                    let mut t = Test::default();
                    let mut kind = PropertyKind::default();

                    parse_attrs!(tokens, {
                        "name" => kind,
                        "qual" => t.qual,
                        "target" => t.target,
                        "compare" => t.compare,
                    });

                    t.value = kind.make_property(parse_expr(tokens)?);

                    m.tests.push(t);
                }
                "edit" => {
                    let mut e = Edit::default();
                    let mut kind = PropertyKind::default();

                    parse_attrs!(tokens, {
                        "name" => kind,
                        "mode" => e.mode,
                        "binding" => e.binding,
                    });

                    e.value = kind.make_property(parse_expr(tokens)?);

                    m.edits.push(e);
                }
                _ => {}
            },
            Token::ElementEnd {
                end: ElementEnd::Close(_, e),
                ..
            } if e.as_str() == "match" => break,
            _ => {}
        }
    }

    Ok(m)
}

fn parse_config<'a>(tokens: &mut impl Iterator<Item = Result<Token<'a>>>) -> Result<Config> {
    let mut config = Config::default();

    loop {
        match try_opt!(tokens.next(), "Expect config")? {
            Token::ElementStart { local, .. } => match local.as_str() {
                "rescan" => {
                    config.rescans.push(parse_string("int", tokens)?.parse()?);
                    take_while_end!(tokens, "rescan");
                }
                _ => {}
            },
            Token::ElementEnd {
                end: ElementEnd::Empty,
                ..
            } => break,
            Token::ElementEnd {
                end: ElementEnd::Close(_, e),
                ..
            } if e.as_str() == "config" => break,
            _ => {}
        }
    }

    Ok(config)
}

#[test]
fn test_parse_match() {
    let mut tokens = xmlparser::Tokenizer::from(
        r#"
<match target="font">
  <test name="outline" compare="eq">
    <bool>false</bool>
  </test>
  <edit name="pixelsizefixupfactor" mode="assign">
    <divide>
      <name target="pattern">pixelsize</name>
      <name target="font"   >pixelsize</name>
    </divide>
  </edit>
</match>
    "#,
    )
    .into_iter()
    .map(|r| r.map_err(Into::into));
    parse_match(&mut tokens).unwrap();
}
