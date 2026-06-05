use criterion::{Criterion, criterion_group, criterion_main};
use icu::collator::Collator;
use icu::collator::options::CollatorOptions;
use icu::locale::locale;
use std::sync::LazyLock;

static MUNICIPALITIES: LazyLock<String> = LazyLock::new(|| {
    std::fs::read_to_string("test-data/destatis-de-municipalities-2024.txt").unwrap()
});
static MUNICIPALITY_RECORDS: LazyLock<String> = LazyLock::new(|| {
    std::fs::read_to_string("test-data/destatis-de-municipality-records-2024.tsv").unwrap()
});

fn lines(data: &'static str) -> Vec<&'static str> {
    data.lines().filter(|line| !line.is_empty()).collect()
}

fn feruca_sort(c: &mut Criterion, name: &str, data: Vec<&'static str>) {
    let mut collator = feruca::Collator::new(feruca::Tailoring::default(), false, false);

    c.bench_function(name, |b| {
        b.iter(|| {
            let mut collected = data.clone();
            collected.sort_unstable_by(|a, b| collator.collate(a, b));
        })
    });
}

fn icu4x_sort(c: &mut Criterion, name: &str, data: Vec<&'static str>) {
    let icu_coll = Collator::try_new(locale!("und").into(), CollatorOptions::default()).unwrap();

    c.bench_function(name, |b| {
        b.iter(|| {
            let mut collected = data.clone();
            collected.sort_unstable_by(|a, b| icu_coll.compare(a, b));
        })
    });
}

fn naive_sort(c: &mut Criterion, name: &str, data: Vec<&'static str>) {
    c.bench_function(name, |b| {
        b.iter(|| {
            let mut collected = data.clone();
            collected.sort_unstable();
        })
    });
}

fn feruca(c: &mut Criterion) {
    feruca_sort(
        c,
        "feruca Destatis-DE municipality names sort",
        lines(&MUNICIPALITIES),
    );
    feruca_sort(
        c,
        "feruca Destatis-DE municipality records sort",
        lines(&MUNICIPALITY_RECORDS),
    );
}

fn icu4x(c: &mut Criterion) {
    icu4x_sort(
        c,
        "icu4x Destatis-DE municipality names sort",
        lines(&MUNICIPALITIES),
    );
    icu4x_sort(
        c,
        "icu4x Destatis-DE municipality records sort",
        lines(&MUNICIPALITY_RECORDS),
    );
}

fn naive(c: &mut Criterion) {
    naive_sort(
        c,
        "naive Destatis-DE municipality names sort",
        lines(&MUNICIPALITIES),
    );
    naive_sort(
        c,
        "naive Destatis-DE municipality records sort",
        lines(&MUNICIPALITY_RECORDS),
    );
}

criterion_group!(benches, feruca, icu4x, naive);
criterion_main!(benches);
