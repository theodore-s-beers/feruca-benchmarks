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

`cargo bench` runs the actual benchmarks.

The `met` benchmark uses derived fixtures from The Metropolitan Museum of Art
Open Access CSV (`MetObjects.csv`): 100,000 non-empty titles, 100,000 non-empty
title/artist display-name pairs, and all unique non-empty artist display names
from the downloaded snapshot. The Met publishes this dataset as CC0:
<https://github.com/metmuseum/openaccess>

The `destatis-de` benchmark uses derived fixtures from the German Federal
Statistical Office municipality register, "Alle politisch selbständigen
Gemeinden mit ausgewählten Merkmalen am 31.12.2024." The benchmark includes all
10,959 municipality-level rows (`Satzart = 60`) as name-only and
municipality/county/state record fixtures. See
<https://www.destatis.de/DE/Themen/Laender-Regionen/Regionales/Gemeindeverzeichnis/Administrativ/Archiv/GVAuszugJ/31122024_Auszug_GV.html>.
