# feruca-benchmarks

This repository exists to benchmark the performance of
[feruca](https://github.com/theodore-s-beers/feruca) – a basic implementation of
the Unicode Collation Algorithm in Rust – against the official ICU4X Rust
collator via the [`icu` crate](https://crates.io/crates/icu). There is also a
comparison to the performance of naïve text sorting based on the byte values of
characters, which cannot handle accents or multiple scripts correctly but is
extremely fast.

`cargo run --release` uses ICU4X and feruca to sort the same text and verify
that they produce identical output.

`cargo bench` runs the actual benchmarks (there are currently four).
