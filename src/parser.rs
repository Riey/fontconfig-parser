use crate::*;
use roxmltree::{Document as XmlDocument, Node};

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

pub fn parse_document(xml_doc: &XmlDocument) -> Result<Document> {
    let mut doc = Document::default();

    let fontconfig = xml_doc.root_element();

    if fontconfig.tag_name().name() != "fontconfig" {
        return Err(Error::NoFontconfig);
    }

    for child in fontconfig.children().filter(|c| c.is_element()) {
        match child.tag_name().name() {
            "description" => {
                doc.description = child
                    .first_child()
                    .and_then(|c| c.text())
                    .map(Into::into)
                    .unwrap_or_default();
            }
            "alias" => {
                let mut alias = Alias::default();

                for child in child.children() {
                    let families =
                        child
                            .children()
                            .filter_map(|family| match family.tag_name().name() {
                                "family" => family.text().map(Into::into),
                                _ => None,
                            });

                    match child.tag_name().name() {
                        "family" => {
                            alias.alias = try_text!(child).into();
                        }
                        "prefer" => {
                            alias.prefer.extend(families);
                        }
                        "accept" => {
                            alias.accept.extend(families);
                        }
                        "default" => {
                            alias.default.extend(families);
                        }
                        _ => {}
                    }
                }

                doc.aliases.push(alias);
            }
            "dir" => {
                let mut dir = Dir::default();

                parse_attrs!(child, {
                    "prefix" => dir.prefix,
                }, {
                    "salt" => dir.salt,
                });

                dir.path = try_text!(child).into();

                doc.dirs.push(dir);
            }
            "cachedir" => {
                let mut dir = CacheDir::default();

                parse_attrs!(child, {
                    "prefix" => dir.prefix,
                });

                dir.path = try_text!(child).into();

                doc.cache_dirs.push(dir);
            }
            "include" => {
                let mut dir = Include::default();
                let mut ignore_missing = "";

                parse_attrs!(child, {
                    "prefix" => dir.prefix,
                }, {
                    "ignore_missing" => ignore_missing,
                });

                dir.ignore_missing = match ignore_missing {
                    "yes" => true,
                    _ => false,
                };

                dir.path = try_text!(child).into();

                doc.includes.push(dir);
            }
            "config" => {
                for child in child.children() {
                    match child.tag_name().name() {
                        "rescan" => {
                            if let Some(int) = child.first_element_child() {
                                if int.tag_name().name() == "int" {
                                    doc.config.rescans.push(try_text!(int).parse()?);
                                }
                            }
                        }
                        "blank" => {}
                        _ => {}
                    }
                }
            }
            "selectfont" => {
                let mut s = SelectFont::default();

                for child in child.children() {
                    let matches = child.children().filter_map(|c| match c.tag_name().name() {
                        "pattern" => {
                            let patelts = c.children().filter_map(|patelt| {
                                if patelt.tag_name().name() == "patelt" {
                                    let mut kind = PropertyKind::default();
                                    parse_attrs_opt!(patelt, {
                                        "name" => kind,
                                    });
                                    parse_expr(patelt.first_element_child()?)
                                        .ok()
                                        .map(|expr| kind.make_property(expr))
                                } else {
                                    None
                                }
                            });
                            Some(FontMatch::Pattern(patelts.collect()))
                        }
                        "glob" => c.text().map(Into::into).map(FontMatch::Glob),
                        _ => None,
                    });

                    match child.tag_name().name() {
                        "acceptfont" => {
                            s.accepts.extend(matches);
                        }
                        "rejectfont" => {
                            s.rejects.extend(matches);
                        }
                        _ => {}
                    }
                }

                doc.select_fonts.push(s);
            }
            "match" => {
                let mut m = Match::default();

                parse_attrs!(child, {
                    "target" => m.target,
                });

                for child in child.children() {
                    match child.tag_name().name() {
                        "test" => {
                            let mut t = Test::default();
                            let mut kind = PropertyKind::default();

                            parse_attrs!(child, {
                                "name" => kind,
                                "qual" => t.qual,
                                "target" => t.target,
                                "compare" => t.compare,
                            });

                            t.value = kind
                                .make_property(parse_expr(child.first_element_child().unwrap())?);

                            m.tests.push(t);
                        }

                        "edit" => {
                            let mut e = Edit::default();
                            let mut kind = PropertyKind::default();

                            parse_attrs!(child, {
                                "name" => kind,
                                "mode" => e.mode,
                                "binding" => e.binding,
                            });

                            e.value = kind
                                .make_property(parse_expr(child.first_element_child().unwrap())?);

                            m.edits.push(e);
                        }
                        _ => {}
                    }
                }

                doc.matches.push(m);
            }
            _ => {
                #[cfg(feature = "std")]
                eprintln!("Ignore {:?}", child.tag_name());
            }
        }
    }

    Ok(doc)
}

fn parse_int_or_range(node: Node) -> Result<IntOrRange> {
    let mut texts = get_texts(&node);

    match node.tag_name().name() {
        "int" => Ok(IntOrRange::Int(try_text!(node).parse()?)),
        "range" => Ok(IntOrRange::Range(
            try_next!(texts, "Expect int").parse()?,
            try_next!(texts, "Expect int").parse()?,
        )),
        _ => Err(Error::InvalidFormat(format!("Expect IntOrRange"))),
    }
}

fn parse_expr(node: Node) -> Result<Expression> {
    let mut exprs = get_exprs(&node);
    let mut texts = get_texts(&node);

    macro_rules! next {
        ($iter:expr) => {
            try_next!($iter, "Expect expression")
        };
    }

    match node.tag_name().name() {
        "string" => return Ok(Value::String(try_text!(node).into()).into()),
        "langset" => return Ok(Value::LangSet(try_text!(node).into()).into()),
        "double" => return Ok(Value::Double(try_text!(node).parse()?).into()),
        "int" => return Ok(Value::Int(try_text!(node).parse()?).into()),
        "bool" => return Ok(Value::Bool(try_text!(node).parse()?).into()),
        "const" => return Ok(Value::Constant(try_text!(node).parse()?).into()),
        "matrix" => {
            return Ok(Expression::Matrix(Box::new([
                next!(exprs)?,
                next!(exprs)?,
                next!(exprs)?,
                next!(exprs)?,
            ])));
        }
        "charset" => {
            let charset = node
                .children()
                .filter_map(|c| parse_int_or_range(c).ok())
                .collect();

            return Ok(Value::CharSet(charset).into());
        }
        "range" => {
            return Ok(Value::Range(next!(texts).parse()?, next!(texts).parse()?).into());
        }
        "name" => {
            let mut target = PropertyTarget::default();
            parse_attrs!(node, {
                "target" => target,
            });
            let kind = try_text!(node).parse()?;

            return Ok(Value::Property(target, kind).into());
        }
        name => {
            return if let Ok(list_op) = name.parse() {
                Ok(Expression::List(
                    list_op,
                    exprs.collect::<Result<Vec<_>>>()?,
                ))
            } else if let Ok(unary_op) = name.parse() {
                Ok(Expression::Unary(unary_op, Box::new(next!(exprs)?)))
            } else if let Ok(binary_op) = name.parse() {
                Ok(Expression::Binary(
                    binary_op,
                    Box::new([next!(exprs)?, next!(exprs)?]),
                ))
            } else if let Ok(ternary_op) = name.parse() {
                Ok(Expression::Ternary(
                    ternary_op,
                    Box::new([next!(exprs)?, next!(exprs)?, next!(exprs)?]),
                ))
            } else {
                todo!("{:?}", name)
            };
        }
    }
}

fn get_exprs<'a>(node: &'a Node) -> impl Iterator<Item = Result<Expression>> + 'a {
    node.children().filter_map(|n| {
        if n.is_element() {
            Some(parse_expr(n))
        } else {
            None
        }
    })
}

fn get_texts<'a>(node: &'a Node) -> impl Iterator<Item = &'a str> {
    node.children()
        .filter_map(|n| if n.is_element() { n.text() } else { None })
}

macro_rules! make_parse_failed_test {
    ($name:ident, $test_fn:ident, $text:expr,) => {
        #[test]
        #[should_panic]
        fn $name() {
            let doc = XmlDocument::parse($text).expect("Parsing xml");
            let node = doc.root_element();
            $test_fn(node).expect("Run parse");
        }
    };
}

macro_rules! make_parse_test {
    ($name:ident, $test_fn:ident, $text:expr, $value:expr,) => {
        #[test]
        fn $name() {
            let doc = XmlDocument::parse($text).expect("Parsing xml");
            let node = doc.root_element();
            let ret = $test_fn(node).expect("Run parse");
            let expected = $value;
            k9::assert_equal!(expected, ret);
        }
    };
}

make_parse_test!(
    test_parse_charset,
    parse_expr,
    "<charset><range><int>0</int><int>123</int></range></charset>",
    Expression::from(vec![IntOrRange::Range(0, 123)]),
);

make_parse_test!(
    test_parse_int,
    parse_expr,
    "<int>123</int>",
    Expression::from(123),
);

make_parse_failed_test!(test_parse_invalid_int, parse_expr, "<int>123f</int>",);

make_parse_test!(
    test_parse_range,
    parse_expr,
    "<range><int>0</int><int>10</int></range>",
    Expression::from(Value::Range(0, 10)),
);

make_parse_failed_test!(
    test_parse_invalid_range,
    parse_expr,
    "<range>0<int>10</int></range>",
);

make_parse_test!(
    test_langset,
    parse_expr,
    "<langset>ko-KR</langset>",
    Expression::from(Value::LangSet("ko-KR".into())),
);
