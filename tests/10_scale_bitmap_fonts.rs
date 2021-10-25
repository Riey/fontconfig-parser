use fontconfig_parser::*;

#[test]
fn scale_10() {
    let doc = parse_document_from_str(include_str!(
        "../test-conf/conf.d/10-scale-bitmap-fonts.conf"
    ))
    .unwrap();

    let expected = Document {
        description: "Bitmap scaling".into(),
        matches: vec![
            Match {
                target: MatchTarget::Font,
                tests: vec![Test {
                    target: TestTarget::default(),
                    qual: TestQual::default(),
                    compare: TestCompare::Eq,
                    value: Property::Outline(false.into()),
                }],
                edits: vec![Edit {
                    mode: EditMode::Assign,
                    binding: EditBinding::default(),
                    value: Property::PixelSizeFixupFactor(Expression::List(
                        ListOp::Divide,
                        vec![
                            (PropertyTarget::Pattern, PropertyKind::PixelSize).into(),
                            (PropertyTarget::Font, PropertyKind::PixelSize).into(),
                        ],
                    )),
                }],
            },
            Match {
                target: MatchTarget::Font,
                tests: vec![
                    Test {
                        compare: TestCompare::Eq,
                        value: Property::Outline(false.into()),
                        ..Default::default()
                    },
                    Test {
                        compare: TestCompare::Eq,
                        value: Property::Scalable(false.into()),
                        ..Default::default()
                    },
                    Test {
                        compare: TestCompare::Eq,
                        value: Property::Hinting(true.into()),
                        ..Default::default()
                    },
                ],
                edits: vec![Edit {
                    binding: EditBinding::default(),
                    mode: EditMode::Assign,
                    value: Property::ScalingNotNeeded(Expression::List(
                        ListOp::And,
                        vec![
                            Expression::Binary(
                                BinaryOp::Less,
                                Box::new([
                                    (PropertyTarget::Default, PropertyKind::PixelSizeFixupFactor)
                                        .into(),
                                    1.2.into(),
                                ]),
                            ),
                            Expression::Binary(
                                BinaryOp::More,
                                Box::new([
                                    (PropertyTarget::Default, PropertyKind::PixelSizeFixupFactor)
                                        .into(),
                                    0.8.into(),
                                ]),
                            ),
                        ],
                    )),
                }],
            },
            Match {
                target: MatchTarget::Font,
                tests: vec![Test {
                    qual: TestQual::Any,
                    target: TestTarget::Default,
                    compare: TestCompare::Eq,
                    value: Property::ScalingNotNeeded(true.into()),
                }],
                edits: vec![Edit {
                    mode: EditMode::Assign,
                    binding: EditBinding::Weak,
                    value: Property::PixelSizeFixupFactor(1.0.into()),
                }],
            },
            Match {
                target: MatchTarget::Font,
                tests: vec![
                    Test {
                        qual: TestQual::Any,
                        target: TestTarget::Default,
                        compare: TestCompare::Eq,
                        value: Property::Outline(false.into()),
                    },
                    Test {
                        qual: TestQual::Any,
                        target: TestTarget::Default,
                        compare: TestCompare::NotEq,
                        value: Property::PixelSizeFixupFactor(1.0.into()),
                    },
                ],
                edits: vec![
                    Edit {
                        mode: EditMode::Assign,
                        binding: EditBinding::Weak,
                        value: Property::Matrix(Expression::List(
                            ListOp::Times,
                            vec![
                                (PropertyTarget::Default, PropertyKind::Matrix).into(),
                                Expression::Matrix(Box::new([
                                    (PropertyTarget::Default, PropertyKind::PixelSizeFixupFactor)
                                        .into(),
                                    0.0.into(),
                                    0.0.into(),
                                    (PropertyTarget::Default, PropertyKind::PixelSizeFixupFactor)
                                        .into(),
                                ])),
                            ],
                        )),
                    },
                    Edit {
                        mode: EditMode::Assign,
                        binding: EditBinding::Weak,
                        value: Property::Size(Expression::List(
                            ListOp::Divide,
                            vec![
                                (PropertyTarget::Default, PropertyKind::Size).into(),
                                (PropertyTarget::Default, PropertyKind::PixelSizeFixupFactor)
                                    .into(),
                            ],
                        )),
                    },
                ],
            },
        ],
        ..Default::default()
    };

    k9::assert_equal!(expected, doc);
}
