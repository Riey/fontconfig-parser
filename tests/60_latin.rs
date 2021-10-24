use fontconfig_parser::*;

#[test]
fn latin() {
    let doc = parse_document_from_str(include_str!("../test-conf/conf.d/60-latin.conf")).unwrap();

    let expected = Document {
        description: "Set preferable fonts for Latin".into(),
        aliases: vec![
            Alias {
                alias: "serif".into(),
                prefer: vec![
                    "DejaVu Serif".into(),
                    "Times New Roman".into(),
                    "Thorndale AMT".into(),
                    "Luxi Serif".into(),
                    "Nimbus Roman No9 L".into(),
                    "Nimbus Roman".into(),
                    "Times".into(),
                ],
                ..Default::default()
            },
            Alias {
                alias: "sans-serif".into(),
                prefer: vec![
                    "DejaVu Sans".into(),
                    "Verdana".into(),
                    "Arial".into(),
                    "Albany AMT".into(),
                    "Luxi Sans".into(),
                    "Nimbus Sans L".into(),
                    "Nimbus Sans".into(),
                    "Helvetica".into(),
                    "Lucida Sans Unicode".into(),
                    "BPG Glaho International".into(),
                    "Tahoma".into(),
                ],
                ..Default::default()
            },
            Alias {
                alias: "monospace".into(),
                prefer: vec![
                    "DejaVu Sans Mono".into(),
                    "Inconsolata".into(),
                    "Andale Mono".into(),
                    "Courier New".into(),
                    "Cumberland AMT".into(),
                    "Luxi Mono".into(),
                    "Nimbus Mono L".into(),
                    "Nimbus Mono".into(),
                    "Nimbus Mono PS".into(),
                    "Courier".into(),
                ],
                ..Default::default()
            },
            Alias {
                alias: "fantasy".into(),
                prefer: vec![
                    "Impact".into(),
                    "Copperplate Gothic Std".into(),
                    "Cooper Std".into(),
                    "Bauhaus Std".into(),
                ],
                ..Default::default()
            },
            Alias {
                alias: "cursive".into(),
                prefer: vec![
                    "ITC Zapf Chancery Std".into(),
                    "Zapfino".into(),
                    "Comic Sans MS".into(),
                ],
                ..Default::default()
            },
            Alias {
                alias: "system-ui".into(),
                prefer: vec![
                    "Cantarell".into(),
                    "Noto Sans UI".into(),
                    "Segoe UI".into(),
                    "Segoe UI Historic".into(),
                    "Segoe UI Symbol".into(),
                ],
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    k9::assert_equal!(expected, doc);
}
