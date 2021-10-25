use fontconfig_parser::*;

#[test]
fn fonts() {
    let doc = parse_document_from_str(include_str!("../test-conf/fonts.conf")).unwrap();

    let expected = Document {
        description: "Default configuration file".into(),
        matches: vec![
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
            },
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
            },
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
            },
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
            },
        ],
        config: Config { rescans: vec![30], blanks: vec![] },
        cache_dirs: vec![
            CacheDir {
                prefix: DirPrefix::Xdg,
                path: "fontconfig".into(),
            },
            CacheDir {
                prefix: DirPrefix::Default,
                path: "/var/cache/fontconfig".into(),
            },
        ],
        includes: vec![Include {
            prefix: DirPrefix::Relative,
            ignore_missing: true,
            path: "conf.d".into(),
        }],
        dirs: vec![
            Dir {
                prefix: DirPrefix::Xdg,
                salt: "".into(),
                path: "fonts".into(),
            },
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "/nix/store/n3imjg12pw7z9qm8r1wq4sq1x8d2nx0y-dejavu-fonts-minimal-2.37"
                    .into(),
            },
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "~/.nix-profile/lib/X11/fonts".into(),
            },
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "~/.nix-profile/share/fonts".into(),
            },
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "/usr/share/fonts".into(),
            },
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "/usr/local/share/fonts".into(),
            },
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "/nix/var/nix/profiles/default/lib/X11/fonts".into(),
            },
            Dir {
                prefix: DirPrefix::Default,
                salt: "".into(),
                path: "/nix/var/nix/profiles/default/share/fonts".into(),
            },
        ],
        ..Default::default()
    };

    k9::assert_equal!(expected, doc);
}
