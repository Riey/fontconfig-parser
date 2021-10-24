use fontconfig_parser::*;

#[test]
fn nixos_reject_types1() {
    let doc = parse_document_from_str(include_str!(
        "../test-conf/conf.d/53-nixos-reject-type1.conf"
    ))
    .unwrap();

    let expected = Document {
        select_fonts: vec![SelectFont {
            accepts: vec![],
            rejects: vec![FontMatch::Pattern(vec![Property::Fontformat(
                "Type 1".into(),
            )])],
        }],
        ..Default::default()
    };

    k9::assert_equal!(expected, doc);
}
