#![warn(clippy::pedantic, clippy::nursery)]

use icu::collator::Collator;
use icu::collator::options::CollatorOptions;
use icu::locale::locale;

fn assert_same_sort(label: &str, data: &str) {
    let mut collected: Vec<&str> = data.lines().filter(|line| !line.is_empty()).collect();
    let mut cloned = collected.clone();

    let mut fer_coll = feruca::Collator::new(feruca::Tailoring::default(), false, false);
    let icu_coll = Collator::try_new(locale!("und").into(), CollatorOptions::default()).unwrap();

    collected.sort_unstable_by(|a, b| fer_coll.collate(a, b));
    cloned.sort_unstable_by(|a, b| icu_coll.compare(a, b));

    println!("Comparing feruca and icu4x sort results for {label}...");
    assert_eq!(collected, cloned);
    println!("{label} results match!");
}

fn main() {
    let municipalities =
        std::fs::read_to_string("test-data/destatis-de-municipalities-2024.txt").unwrap();
    let records =
        std::fs::read_to_string("test-data/destatis-de-municipality-records-2024.tsv").unwrap();

    assert_same_sort("Destatis-DE municipality names", &municipalities);
    assert_same_sort("Destatis-DE municipality records", &records);
}
