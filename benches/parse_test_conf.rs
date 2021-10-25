use criterion::{criterion_group, criterion_main, Criterion};

fn parse_test(c: &mut Criterion) {
    c.bench_function("parse_full", |b| {
        b.iter(|| {
            let s = std::fs::read_to_string("test-conf/fonts.conf").unwrap();
            fontconfig_parser::parse_config_parts(&s).unwrap();
        });
    });
}

criterion_group!(benches, parse_test);
criterion_main!(benches);
