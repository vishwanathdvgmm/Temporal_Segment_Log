use criterion::{criterion_group, criterion_main, Criterion};
use tsl::{Event, TSL};

fn bench_ingest(c: &mut Criterion) {
    c.bench_function("ingest_1M", |b| {
        b.iter(|| {
            let mut tsl = TSL::new(1000);

            for i in 0..1_000_000 {
                tsl.append(Event::new(i, vec![1]));
            }
        })
    });
}

criterion_group!(benches, bench_ingest);
criterion_main!(benches);
