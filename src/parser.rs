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
                        "blank" => todo!("blank"),
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

fn parse_expr(node: Node) -> Result<Expression> {
    let mut exprs = node.children().filter_map(|n| {
        if n.is_element() {
            Some(parse_expr(n))
        } else {
            None
        }
    });

    macro_rules! expr {
        () => {
            match exprs.next() {
                Some(expr) => expr?,
                None => return Err(Error::InvalidFormat("Expect expression".into())),
            }
        };
    }

    match node.tag_name().name() {
        "string" => return Ok(Value::String(try_text!(node).into()).into()),
        "double" => return Ok(Value::Double(try_text!(node).parse()?).into()),
        "int" => return Ok(Value::Int(try_text!(node).parse()?).into()),
        "bool" => return Ok(Value::Bool(try_text!(node).parse()?).into()),
        "const" => return Ok(Value::Constant(try_text!(node).parse()?).into()),
        "matrix" => {
            return Ok(Expression::Matrix(Box::new([
                expr!(),
                expr!(),
                expr!(),
                expr!(),
            ])));
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
                    exprs.collect::<Result<Vec<_>>>()?.into_boxed_slice(),
                ))
            } else if let Ok(unary_op) = name.parse() {
                Ok(Expression::Unary(unary_op, Box::new(expr!())))
            } else if let Ok(binary_op) = name.parse() {
                Ok(Expression::Binary(binary_op, Box::new([expr!(), expr!()])))
            } else if let Ok(ternary_op) = name.parse() {
                Ok(Expression::Ternary(
                    ternary_op,
                    Box::new([expr!(), expr!(), expr!()]),
                ))
            } else {
                todo!("{:?}", name)
            };
        }
    }
}
