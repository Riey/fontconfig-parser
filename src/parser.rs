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
                    .map(String::from)
                    .unwrap_or_default();
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
            other => {
                #[cfg(feature = "std")]
                eprintln!("Ignore {}", other);
            }
        }
    }

    Ok(doc)
}

fn parse_expr(node: Node) -> Result<Expression> {
    match node.tag_name().name() {
        "string" => return Ok(Value::String(try_text!(node).into()).into()),
        "double" => return Ok(Value::Double(try_text!(node).parse()?).into()),
        "int" => return Ok(Value::Int(try_text!(node).parse()?).into()),
        "bool" => return Ok(Value::Bool(try_text!(node).parse()?).into()),
        "const" => return Ok(Value::Constant(try_text!(node).parse()?).into()),
        "matrix" => {
            let list = node
                .children()
                .filter_map(|n| {
                    if n.is_element() {
                        Some(parse_expr(n))
                    } else {
                        None
                    }
                })
                .collect::<Result<Vec<_>>>()?;

            return Ok(Expression::Matrix(list));
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
            let list = node
                .children()
                .filter_map(|n| {
                    if n.is_element() {
                        Some(parse_expr(n))
                    } else {
                        None
                    }
                })
                .collect::<Result<Vec<_>>>()?;

            return if let Ok(list_op) = name.parse() {
                Ok(Expression::List(list_op, list))
            } else if let Ok(unary_op) = name.parse() {
                Ok(Expression::Unary(unary_op, list))
            } else if let Ok(binary_op) = name.parse() {
                Ok(Expression::Binary(binary_op, list))
            } else if let Ok(ternary_op) = name.parse() {
                Ok(Expression::Ternary(ternary_op, list))
            } else {
                todo!("{:?}", name)
            };
        }
    }
}
//
// fn parse_selectfont<'a>(
//     tokens: &mut impl Iterator<Item = Result<Token<'a>>>,
// ) -> Result<SelectFont<'a>> {
//     let mut s = SelectFont::default();
//
//     loop {
//         match try_opt!(tokens.next(), "Expect selectfont")? {
//             Token::ElementStart { local, .. } => match local.as_str() {
//                 "acceptfont" => loop {
//                     match try_opt!(tokens.next(), "Expect acceptfont")? {
//                         Token::ElementStart { local, .. } => match local.as_str() {
//                             "glob" => s.accepts.push(parse_glob(tokens)?),
//                             "pattern" => s.accepts.push(parse_pattern(tokens)?),
//                             _ => {}
//                         },
//                         Token::ElementEnd {
//                             end: ElementEnd::Close(_, e),
//                             ..
//                         } if e.as_str() == "acceptfont" => break,
//                         _ => {}
//                     }
//                 },
//                 "rejectfont" => loop {
//                     match try_opt!(tokens.next(), "Expect rejectfont")? {
//                         Token::ElementStart { local, .. } => match local.as_str() {
//                             "glob" => s.rejects.push(parse_glob(tokens)?),
//                             "pattern" => s.rejects.push(parse_pattern(tokens)?),
//                             _ => {}
//                         },
//                         Token::ElementEnd {
//                             end: ElementEnd::Close(_, e),
//                             ..
//                         } if e.as_str() == "rejectfont" => break,
//                         _ => {}
//                     }
//                 },
//                 _ => {}
//             },
//             Token::ElementEnd {
//                 end: ElementEnd::Close(_, e),
//                 ..
//             } if e.as_str() == "selectfont" => break,
//             _ => {}
//         }
//     }
//
//     Ok(s)
// }
//
// fn parse_glob<'a>(tokens: &mut impl Iterator<Item = Result<Token<'a>>>) -> Result<FontMatch<'a>> {
//     Ok(FontMatch::Glob(try_text!(child_root)?))
// }
//
// fn parse_pattern<'a>(
//     tokens: &mut impl Iterator<Item = Result<Token<'a>>>,
// ) -> Result<FontMatch<'a>> {
//     let mut patterns = Vec::new();
//
//     loop {
//         match try_opt!(tokens.next(), "Expect pattern")? {
//             Token::ElementStart { local, .. } => match local.as_str() {
//                 "patelt" => {
//                     let mut kind = PropertyKind::default();
//
//                     parse_attrs!(tokens, {
//                         "name" => kind,
//                     });
//
//                     patterns.push(kind.make_property(parse_expr(tokens)?));
//                 }
//                 _ => {}
//             },
//             Token::ElementEnd {
//                 end: ElementEnd::Close(_, e),
//                 ..
//             } if e.as_str() == "pattern" => break,
//             _ => {}
//         }
//     }
//
//     Ok(FontMatch::Pattern(patterns))
// }
