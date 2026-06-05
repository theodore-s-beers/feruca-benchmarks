use criterion::{Criterion, criterion_group, criterion_main};
#[cfg(feature = "pipeline-stats")]
use feruca::PipelineStats;
use icu::collator::Collator;
use icu::collator::options::CollatorOptions;
use icu::locale::locale;
use std::sync::LazyLock;

static MET_TITLES: LazyLock<String> =
    LazyLock::new(|| std::fs::read_to_string("test-data/met-titles-100k.txt").unwrap());
static MET_ARTISTS: LazyLock<String> =
    LazyLock::new(|| std::fs::read_to_string("test-data/met-artists-unique.txt").unwrap());
static MET_TITLE_ARTIST: LazyLock<String> =
    LazyLock::new(|| std::fs::read_to_string("test-data/met-title-artist-100k.tsv").unwrap());

fn lines(data: &'static str) -> Vec<&'static str> {
    data.lines().filter(|line| !line.is_empty()).collect()
}

#[cfg(feature = "pipeline-stats")]
fn report_pipeline_stats(name: &str, data: &[&'static str]) {
    let mut collator = feruca::Collator::new(feruca::Tailoring::default(), false, false);
    let mut collected = data.to_vec();

    collected.sort_unstable_by(|a, b| collator.collate(a, b));
    print_pipeline_stats(name, collator.stats());
}

#[cfg(feature = "pipeline-stats")]
fn benchmark_selected(name: &str) -> bool {
    let filters: Vec<_> = std::env::args()
        .skip(1)
        .filter(|arg| !arg.starts_with("--"))
        .collect();

    filters.is_empty()
        || filters
            .iter()
            .any(|filter| name.contains(filter) || filter.contains(name))
}

#[cfg(feature = "pipeline-stats")]
fn print_pipeline_stats(name: &str, stats: &PipelineStats) {
    eprintln!();
    eprintln!("{name} pipeline stats");
    eprintln!("  comparisons: {}", stats.comparisons);
    eprintln!(
        "  resolved: equal={} ascii-primary={} lazy-utf8-primary={} fill-ascii={} initial-primary={} streaming-primary={} later-levels={} tiebreak={}",
        stats.equal_early,
        stats.ascii_primary_resolved,
        stats.lazy_utf8_primary_resolved,
        stats.fill_ascii_resolved,
        stats.initial_primary_resolved,
        stats.streaming_primary_resolved,
        stats.later_levels_resolved,
        stats.tiebreak_resolved
    );
    eprintln!(
        "  lazy UTF-8 primary attempts: {}",
        stats.lazy_utf8_primary_attempts
    );
    eprintln!(
        "  lazy UTF-8 fallbacks: reused-prefix={} full={}",
        stats.lazy_utf8_prefix_reused, stats.lazy_utf8_full_fallback
    );
    eprintln!(
        "  reached later levels: {} ({:.2}%)",
        stats.later_levels_reached,
        percent(stats.later_levels_reached, stats.comparisons)
    );
    eprintln!(
        "  prefix trims: byte={} ({} bytes) codepoint={} ({} code points)",
        stats.byte_prefix_trimmed,
        stats.byte_prefix_bytes_trimmed,
        stats.codepoint_prefix_trimmed,
        stats.codepoint_prefix_codepoints_trimmed
    );
    eprintln!(
        "  decoded code points: {} ({:.2} per comparison)",
        stats.codepoints_decoded,
        average(stats.codepoints_decoded, stats.comparisons)
    );
    eprintln!(
        "  primary-consumed code points: {} ({:.2} per streaming-primary comparison)",
        stats.codepoints_consumed_primary,
        average(
            stats.codepoints_consumed_primary,
            stats.streaming_primary_resolved + stats.later_levels_reached
        )
    );
    eprintln!(
        "  decoded / primary-consumed: {:.2}",
        ratio(stats.codepoints_decoded, stats.codepoints_consumed_primary)
    );
    eprintln!("  NFD normalizations: {}", stats.nfd_normalizations);
}

#[cfg(feature = "pipeline-stats")]
fn percent(numerator: u64, denominator: u64) -> f64 {
    ratio(numerator, denominator) * 100.0
}

#[cfg(feature = "pipeline-stats")]
fn average(numerator: u64, denominator: u64) -> f64 {
    ratio(numerator, denominator)
}

#[cfg(feature = "pipeline-stats")]
fn ratio(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn feruca_sort(c: &mut Criterion, name: &str, data: Vec<&'static str>) {
    #[cfg(feature = "pipeline-stats")]
    if benchmark_selected(name) {
        report_pipeline_stats(name, &data);
    }

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
    feruca_sort(c, "feruca Met titles sort", lines(&MET_TITLES));
    feruca_sort(c, "feruca Met artists sort", lines(&MET_ARTISTS));
    feruca_sort(c, "feruca Met title/artist sort", lines(&MET_TITLE_ARTIST));
}

fn icu4x(c: &mut Criterion) {
    icu4x_sort(c, "icu4x Met titles sort", lines(&MET_TITLES));
    icu4x_sort(c, "icu4x Met artists sort", lines(&MET_ARTISTS));
    icu4x_sort(c, "icu4x Met title/artist sort", lines(&MET_TITLE_ARTIST));
}

fn naive(c: &mut Criterion) {
    naive_sort(c, "naive Met titles sort", lines(&MET_TITLES));
    naive_sort(c, "naive Met artists sort", lines(&MET_ARTISTS));
    naive_sort(c, "naive Met title/artist sort", lines(&MET_TITLE_ARTIST));
}

criterion_group!(benches, feruca, icu4x, naive);
criterion_main!(benches);
