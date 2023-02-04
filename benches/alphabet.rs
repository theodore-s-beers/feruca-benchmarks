use criterion::{criterion_group, criterion_main, Criterion};
use feruca::{Collator, Tailoring};
use rust_icu_ucol as ucol;
use std::convert::TryFrom;

const ALPHABET: [&str; 5_616] = [
    "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h",
    "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o",
    "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v",
    "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c",
    "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j",
    "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q",
    "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x",
    "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e",
    "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l",
    "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s",
    "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z",
    "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g",
    "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n",
    "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u",
    "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b",
    "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i",
    "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p",
    "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w",
    "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d",
    "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k",
    "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r",
    "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y",
    "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f",
    "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m",
    "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t",
    "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a",
    "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h",
    "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o",
    "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v",
    "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c",
    "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j",
    "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q",
    "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x",
    "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e",
    "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l",
    "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s",
    "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z",
    "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g",
    "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n",
    "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u",
    "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b",
    "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i",
    "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p",
    "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w",
    "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d",
    "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k",
    "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r",
    "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y",
    "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f",
    "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m",
    "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t",
    "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a",
    "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h",
    "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o",
    "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v",
    "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c",
    "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j",
    "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q",
    "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x",
    "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e",
    "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l",
    "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s",
    "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z",
    "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g",
    "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n",
    "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u",
    "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b",
    "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i",
    "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p",
    "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w",
    "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d",
    "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k",
    "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r",
    "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y",
    "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f",
    "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m",
    "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t",
    "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a",
    "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h",
    "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o",
    "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v",
    "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c",
    "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j",
    "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q",
    "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x",
    "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e",
    "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l",
    "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s",
    "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z",
    "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g",
    "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n",
    "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u",
    "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b",
    "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i",
    "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p",
    "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w",
    "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d",
    "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k",
    "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r",
    "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y",
    "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f",
    "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m",
    "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t",
    "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a",
    "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h",
    "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o",
    "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v",
    "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c",
    "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j",
    "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q",
    "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x",
    "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e",
    "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l",
    "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s",
    "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z",
    "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g",
    "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n",
    "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u",
    "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b",
    "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i",
    "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p",
    "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w",
    "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d",
    "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k",
    "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r",
    "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y",
    "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f",
    "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m",
    "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t",
    "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a",
    "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h",
    "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o",
    "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v",
    "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c",
    "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j",
    "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q",
    "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x",
    "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e",
    "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l",
    "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s",
    "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z",
    "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g",
    "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n",
    "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u",
    "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b",
    "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i",
    "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p",
    "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w",
    "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d",
    "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k",
    "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r",
    "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y",
    "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f",
    "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m",
    "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t",
    "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a",
    "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h",
    "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o",
    "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v",
    "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c",
    "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j",
    "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q",
    "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x",
    "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e",
    "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l",
    "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s",
    "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z",
    "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g",
    "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n",
    "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u",
    "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b",
    "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i",
    "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p",
    "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w",
    "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d",
    "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k",
    "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r",
    "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y",
    "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f",
    "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m",
    "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t",
    "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a",
    "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h",
    "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o",
    "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v",
    "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c",
    "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j",
    "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q",
    "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x",
    "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e",
    "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l",
    "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s",
    "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z",
    "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g",
    "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n",
    "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u",
    "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b",
    "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i",
    "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p",
    "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w",
    "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d",
    "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k",
    "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r",
    "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y",
    "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f",
    "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m",
    "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t",
    "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a",
    "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h",
    "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o",
    "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v",
    "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c",
    "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j",
    "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q",
    "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x",
    "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e",
    "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l",
    "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s",
    "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z",
    "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g",
    "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n",
    "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u",
    "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b",
    "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i",
    "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p",
    "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w",
    "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d",
    "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k",
    "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r",
    "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y",
    "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f",
    "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m",
    "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t",
    "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a",
    "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h",
    "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o",
    "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v",
    "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c",
    "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j",
    "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q",
    "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x",
    "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e",
    "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l",
    "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s",
    "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z",
    "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g",
    "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n",
    "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u",
    "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b",
    "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i",
    "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p",
    "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w",
    "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d",
    "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k",
    "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r",
    "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y",
    "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f",
    "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m",
    "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t",
    "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a",
    "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h",
    "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o",
    "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v",
    "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c",
    "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j",
    "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q",
    "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x",
    "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e",
    "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l",
    "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s",
    "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z",
    "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g",
    "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n",
    "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u",
    "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b",
    "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i",
    "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p",
    "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w",
    "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d",
    "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k",
    "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r",
    "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y",
    "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f",
    "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m",
    "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t",
    "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a",
    "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h",
    "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o",
    "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v",
    "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c",
    "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j",
    "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q",
    "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a", "z", "y", "x",
    "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l", "k", "j", "i", "h", "g", "f", "e",
    "d", "c", "b", "a", "z", "y", "x", "w", "v", "u", "t", "s", "r", "q", "p", "o", "n", "m", "l",
    "k", "j", "i", "h", "g", "f", "e", "d", "c", "b", "a",
];

fn feruca(c: &mut Criterion) {
    let mut collator = Collator::new(Tailoring::default(), false);

    c.bench_function("feruca alphabet sort", |b| {
        b.iter(|| {
            let mut al = ALPHABET;
            al.sort_unstable_by(|a, b| collator.collate_no_tiebreak(a, b));
        })
    });
}

fn ucol(c: &mut Criterion) {
    let collator = ucol::UCollator::try_from("en").expect("collator");

    c.bench_function("ucol alphabet sort", |b| {
        b.iter(|| {
            let mut al = ALPHABET;
            al.sort_unstable_by(|a, b| collator.strcoll_utf8(a, b).unwrap());
        })
    });
}

fn naive(c: &mut Criterion) {
    c.bench_function("naive alphabet sort", |b| {
        b.iter(|| {
            let mut al = ALPHABET;
            al.sort_unstable();
        })
    });
}

criterion_group!(benches, feruca, ucol, naive);
criterion_main!(benches);
