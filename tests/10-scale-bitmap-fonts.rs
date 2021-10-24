use fontconfig_parser::*;
use pretty_assertions::assert_eq;

#[test]
fn scale_10() {
    let doc = parse_document_from_str(include_str!(
        "../test-conf/conf.d/10-scale-bitmap-fonts.conf"
    ))
    .unwrap();

    dbg!(&doc);

    assert_eq!(doc.description, "Bitmap scaling");
    assert_eq!(
        doc.matches[0],
        Match {
            target: MatchTarget::Font,
            tests: vec![Test {
                target: TestTarget::default(),
                qual: TestQual::default(),
                compare: TestCompare::Eq,
                value: Property::Outline(Value::Bool(false).into()),
            },],
            edits: vec![Edit {
                mode: EditMode::Assign,
                binding: EditBinding::default(),
                value: Property::PixelSizeFixupFactor(Expression::List(
                    ListOp::Divide,
                    vec![
                        Value::Property(PropertyTarget::Pattern, PropertyKind::PixelSize).into(),
                        Value::Property(PropertyTarget::Font, PropertyKind::PixelSize).into(),
                    ]
                )),
            }],
        }
    );

    assert_eq!(
        doc.matches[1],
        Match {
            target: MatchTarget::Font,
            tests: vec![
                Test {
                    compare: TestCompare::Eq,
                    value: Property::Outline(Value::Bool(false).into()),
                    ..Default::default()
                },
                Test {
                    compare: TestCompare::Eq,
                    value: Property::Scalable(Value::Bool(false).into()),
                    ..Default::default()
                },
                Test {
                    compare: TestCompare::Eq,
                    value: Property::Hinting(Value::Bool(true).into()),
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
                            vec![
                                Value::Property(
                                    PropertyTarget::Default,
                                    PropertyKind::PixelSizeFixupFactor
                                )
                                .into(),
                                Value::Double(1.2).into(),
                            ],
                        ),
                        Expression::Binary(
                            BinaryOp::More,
                            vec![
                                Value::Property(
                                    PropertyTarget::Default,
                                    PropertyKind::PixelSizeFixupFactor
                                )
                                .into(),
                                Value::Double(0.8).into(),
                            ],
                        ),
                    ],
                ))
            }]
        },
    );
}
