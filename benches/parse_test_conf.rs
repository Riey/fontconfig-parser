use criterion::{criterion_group, criterion_main, Criterion};

fn parse_test(c: &mut Criterion) {
    c.bench_function("parse_full", |b| {
        b.iter(|| {
            let mut reader = fontconfig_parser::DocumentReader::new();
            reader
                .read_document(&mut quick_xml::Reader::from_file("test-conf/fonts.conf").unwrap())
                .unwrap();
        });
    });
}

criterion_group!(benches, parse_test);
criterion_main!(benches);
