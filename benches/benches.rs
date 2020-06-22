/* project use */
use cocktail;

/* criterion use */
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn rev(c: &mut Criterion) {
    let mut group = c.benchmark_group("rev");
    for k in (1..32).step_by(2) {
        group.bench_with_input(BenchmarkId::new("pub", k), &k, |b, &k| {
            b.iter(|| cocktail::kmer::rev(black_box(1445814085), black_box(k)))
        });
        group.bench_with_input(BenchmarkId::new("loop", k), &k, |b, &k| {
            b.iter(|| cocktail::kmer::loop_rev(black_box(1445814085), black_box(k)))
        });
        group.bench_with_input(BenchmarkId::new("unrool", k), &k, |b, &k| {
            b.iter(|| cocktail::kmer::unrool_rev(black_box(1445814085), black_box(k)))
        });
    }
}

mod kmer2seq;

fn kmer2seq(c: &mut Criterion) {
    assert_eq!(
        kmer2seq::static_buffer(7271, 5),
        cocktail::kmer::kmer2seq(7271, 5)
    );
    assert_eq!(
        kmer2seq::static_buffer(7271, 5),
        kmer2seq::local_buffer(7271, 5)
    );
    assert_eq!(
        kmer2seq::static_buffer(7271, 5),
        kmer2seq::dyn_local_buffer(7271, 5)
    );

    let mut g = c.benchmark_group("kmer2seq");

    for k in (1..32).step_by(2) {
        g.bench_with_input(BenchmarkId::new("actual implementation", k), &k, |b, &k| {
            b.iter(|| black_box(cocktail::kmer::kmer2seq(black_box(7271), k)))
        });

        g.bench_with_input(BenchmarkId::new("static buffer", k), &k, |b, &k| {
            b.iter(|| black_box(kmer2seq::static_buffer(black_box(7271), k)))
        });

        g.bench_with_input(BenchmarkId::new("local buffer", k), &k, |b, &k| {
            b.iter(|| black_box(kmer2seq::local_buffer(black_box(7271), k)))
        });

        g.bench_with_input(BenchmarkId::new("dynamic local buffer", k), &k, |b, &k| {
            b.iter(|| black_box(kmer2seq::dyn_local_buffer(black_box(7271), k)))
        });
    }
}

fn setup(c: &mut Criterion) {
    kmer2seq(c);
    rev(c);
}

criterion_group!(benches, setup);

criterion_main!(benches);
