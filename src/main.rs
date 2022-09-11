#![warn(clippy::pedantic, clippy::nursery)]

use feruca::{Collator, Tailoring};
use rust_icu_ucol as ucol;
use std::convert::TryFrom;

fn main() {
    let german = std::fs::read_to_string("test-data/mars-de.txt").unwrap();

    let mut collected: Vec<&str> = german.split_whitespace().collect();
    let mut cloned = collected.clone();

    let mut fer_coll = Collator::new(Tailoring::default(), false);
    let collator = ucol::UCollator::try_from("en").unwrap();

    collected.sort_unstable_by(|a, b| fer_coll.collate_no_tiebreak(a, b));
    cloned.sort_unstable_by(|a, b| collator.strcoll_utf8(a, b).unwrap());

    assert_eq!(cloned, collected);
}
