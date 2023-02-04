use criterion::{criterion_group, criterion_main, Criterion};
use feruca::{Collator, Tailoring};
use once_cell::sync::Lazy;
use rust_icu_ucol as ucol;
use std::convert::TryFrom;

static MEL: Lazy<String> =
    Lazy::new(|| std::fs::read_to_string("test-data/melania-sl.txt").unwrap());

fn feruca(c: &mut Criterion) {
    let mut collator = Collator::new(Tailoring::default(), false);

    c.bench_function("feruca Melania-SL text sort", |b| {
        b.iter(|| {
            let text = MEL.clone();
            let mut collected: Vec<&str> = text.split_whitespace().collect();
            collected.sort_unstable_by(|a, b| collator.collate_no_tiebreak(a, b));
        })
    });
}

fn ucol(c: &mut Criterion) {
    let collator = ucol::UCollator::try_from("en").expect("collator");

    c.bench_function("ucol Melania-SL text sort", |b| {
        b.iter(|| {
            let text = MEL.clone();
            let mut collected: Vec<&str> = text.split_whitespace().collect();
            collected.sort_unstable_by(|a, b| collator.strcoll_utf8(a, b).unwrap());
        })
    });
}

fn naive(c: &mut Criterion) {
    c.bench_function("naive Melania-SL text sort", |b| {
        b.iter(|| {
            let text = MEL.clone();
            let mut collected: Vec<&str> = text.split_whitespace().collect();
            collected.sort_unstable();
        })
    });
}

criterion_group!(benches, feruca, ucol, naive);
criterion_main!(benches);
