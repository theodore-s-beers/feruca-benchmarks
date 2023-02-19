#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::string_lit_as_bytes)]

use icu::collator::{Collator, CollatorOptions};
use icu::locid::locale;

fn main() {
    let german = std::fs::read_to_string("test-data/mars-de.txt").unwrap();

    let mut collected: Vec<&str> = german.split_whitespace().collect();
    let mut cloned = collected.clone();

    let mut fer_coll = feruca::Collator::new(feruca::Tailoring::default(), false);
    let icu_coll = Collator::try_new_unstable(
        &icu_testdata::unstable(),
        &locale!("en").into(),
        CollatorOptions::new(),
    )
    .unwrap();

    collected.sort_unstable_by(|a, b| fer_coll.collate_no_tiebreak(a, b));
    cloned.sort_unstable_by(|a, b| icu_coll.compare(a, b));

    assert_eq!(collected, cloned);
}
