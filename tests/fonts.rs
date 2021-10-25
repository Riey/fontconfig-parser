use fontconfig_parser::*;

/*
#[test]
fn fonts() {
    let configs = parse_config_parts(include_str!("../test-conf/fonts.conf")).unwrap();

    let expected = vec![
        ConfigPart::Description("Default configuration file".into()),
        Match {
                target: MatchTarget::Pattern,
                tests: vec![Test {
                    qual: TestQual::Any,
                    value: Property::Family("mono".into()),
                    ..Default::default()
                }],
                edits: vec![Edit {
                    mode: EditMode::Assign,
                    binding: EditBinding::Same,
                    value: Property::Family("monospace".into()),
                }],
            }.into(),
            Match {
                target: MatchTarget::Pattern,
                tests: vec![Test {
                    qual: TestQual::Any,
                    value: Property::Family("sans serif".into()),
                    ..Default::default()
                }],
                edits: vec![Edit {
                    mode: EditMode::Assign,
                    binding: EditBinding::Same,
                    value: Property::Family("sans-serif".into()),
                }],
            }.into(),
            Match {
                target: MatchTarget::Pattern,
                tests: vec![Test {
                    qual: TestQual::Any,
                    value: Property::Family("sans".into()),
                    ..Default::default()
                }],
                edits: vec![Edit {
                    mode: EditMode::Assign,
                    binding: EditBinding::Same,
                    value: Property::Family("sans-serif".into()),
                }],
            }.into(),
            Match {
                target: MatchTarget::Pattern,
                tests: vec![Test {
                    qual: TestQual::Any,
                    value: Property::Family("system ui".into()),
                    ..Default::default()
                }],
                edits: vec![Edit {
                    mode: EditMode::Assign,
                    binding: EditBinding::Same,
                    value: Property::Family("system-ui".into()),
                }],
            }.into(),
       Config {
            rescans: vec![30],
            blanks: vec![IntOrRange::Int(0)],
        }.into(),
        cache_dirs: vec![
            CacheDir {
                prefix: DirPrefix::Xdg,
                path: "fontconfig".into(),
            }.into(),
            CacheDir {
                prefix: DirPrefix::Default,
                path: "/var/cache/fontconfig".into(),
            }.into(),
        ],
        includes: vec![Include {
            prefix: DirPrefix::Relative,
            ignore_missing: true,
            path: "conf.d".into(),
        }.into()],
        dirs: vec![
            Dir {
                prefix: DirPrefix::Xdg,
                salt: "".into(),
                path: "fonts".into(),
            }.into(),
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "/nix/store/n3imjg12pw7z9qm8r1wq4sq1x8d2nx0y-dejavu-fonts-minimal-2.37"
                    .into(),
            }.into(),
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "~/.nix-profile/lib/X11/fonts".into(),
            }.into(),
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "~/.nix-profile/share/fonts".into(),
            }.into(),
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "/usr/share/fonts".into(),
            }.into(),
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "/usr/local/share/fonts".into(),
            }.into(),
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "/nix/var/nix/profiles/default/lib/X11/fonts".into(),
            }.into(),
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "/nix/var/nix/profiles/default/share/fonts".into(),
            }.into(),
        ],
        ..Default::default()
    };

    k9::assert_equal!(expected, doc);
}
*/
