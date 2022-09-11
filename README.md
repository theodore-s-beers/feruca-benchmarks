# feruca-benchmarks

This repository exists simply to benchmark the performance of
[feruca](https://github.com/theodore-s-beers/feruca)—a basic implementation of
the Unicode Collation Algorithm in Rust—against the official C library,
[ucol](https://github.com/unicode-org/icu). There is also a comparison to the
performance of naïve text sorting based on the byte values of characters, which
cannot handle accents or multiple scripts correctly but is very, very fast.

My testing so far suggests that sorting accented text with `ucol` is on the
order of 3 times slower than sorting by byte value. `feruca`, in turn, is on the
order of 5 times slower than `ucol`. Not great, but not terrible.

In order to run these benchmarks, you will need to have `icu4c` installed on
your system. That can be a surprisingly complicated process, so, best of luck…

Assuming you have the necessary dependencies available, `cargo run --release`
will use `ucol` and `feruca` to sort the same text and verify that they produce
identical output. `cargo bench` will run the actual benchmarks (there are
currently four).
