use criterion::{Criterion, criterion_group, criterion_main};
use icu::collator::{Collator, CollatorOptions};
use icu::locid::locale;
use std::sync::LazyLock;

static MARS: LazyLock<String> =
    LazyLock::new(|| std::fs::read_to_string("test-data/mars-de.txt").unwrap());

fn feruca(c: &mut Criterion) {
    let mut collator = feruca::Collator::new(feruca::Tailoring::default(), false, false);

    c.bench_function("feruca Mars-DE text sort", |b| {
        b.iter(|| {
            let text = MARS.clone();
            let mut collected: Vec<&str> = text.split_whitespace().collect();
            collected.sort_unstable_by(|a, b| collator.collate(a, b));
        })
    });
}

fn ucol(c: &mut Criterion) {
    let icu_coll = Collator::try_new(&locale!("en").into(), CollatorOptions::new()).unwrap();

    c.bench_function("ucol Mars-DE text sort", |b| {
        b.iter(|| {
            let text = MARS.clone();
            let mut collected: Vec<&str> = text.split_whitespace().collect();
            collected.sort_unstable_by(|a, b| icu_coll.compare(a, b));
        })
    });
}

fn naive(c: &mut Criterion) {
    c.bench_function("naive Mars-DE text sort", |b| {
        b.iter(|| {
            let text = MARS.clone();
            let mut collected: Vec<&str> = text.split_whitespace().collect();
            collected.sort_unstable();
        })
    });
}

criterion_group!(benches, feruca, ucol, naive);
criterion_main!(benches);
