#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::string_lit_as_bytes)]

use icu::collator::Collator;
use icu::collator::options::CollatorOptions;
use icu::locale::locale;

fn main() {
    let german = std::fs::read_to_string("test-data/mars-de.txt").unwrap();

    let mut collected: Vec<&str> = german.split_whitespace().collect();
    let mut cloned = collected.clone();

    let mut fer_coll = feruca::Collator::new(feruca::Tailoring::default(), false, false);
    let icu_coll = Collator::try_new(locale!("en").into(), CollatorOptions::default()).unwrap();

    collected.sort_unstable_by(|a, b| fer_coll.collate(a, b));
    cloned.sort_unstable_by(|a, b| icu_coll.compare(a, b));

    println!("Comparing feruca and icu4x sort results...");
    assert_eq!(collected, cloned);
    println!("Results match!");
}
