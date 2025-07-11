use criterion::{Criterion, criterion_group, criterion_main};
use icu::collator::Collator;
use icu::collator::options::CollatorOptions;
use icu::locale::locale;
use std::sync::LazyLock;

static MARS: LazyLock<String> =
    LazyLock::new(|| std::fs::read_to_string("test-data/mars-de.txt").unwrap());

fn feruca(c: &mut Criterion) {
    let mut collator = feruca::Collator::new(feruca::Tailoring::default(), false, false);
    let data: Vec<&str> = MARS.split_whitespace().collect();

    c.bench_function("feruca Mars-DE text sort", |b| {
        b.iter(|| {
            let mut collected = data.clone();
            collected.sort_unstable_by(|a, b| collator.collate(a, b));
        })
    });
}

fn ucol(c: &mut Criterion) {
    let icu_coll = Collator::try_new(locale!("en").into(), CollatorOptions::default()).unwrap();
    let data: Vec<&str> = MARS.split_whitespace().collect();

    c.bench_function("ucol Mars-DE text sort", |b| {
        b.iter(|| {
            let mut collected = data.clone();
            collected.sort_unstable_by(|a, b| icu_coll.compare(a, b));
        })
    });
}

fn naive(c: &mut Criterion) {
    let data: Vec<&str> = MARS.split_whitespace().collect();

    c.bench_function("naive Mars-DE text sort", |b| {
        b.iter(|| {
            let mut collected = data.clone();
            collected.sort_unstable();
        })
    });
}

criterion_group!(benches, feruca, ucol, naive);
criterion_main!(benches);
