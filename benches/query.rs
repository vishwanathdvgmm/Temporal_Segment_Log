use criterion::{criterion_group, criterion_main, Criterion};
use tsl::{Event, TSL};

fn bench_query(c: &mut Criterion) {
    let mut tsl = TSL::new(1000);

    for i in 0..1_000_000 {
        tsl.append(Event::new(i, vec![1]));
    }

    c.bench_function("query_range", |b| {
        b.iter(|| {
            tsl.range_query(1000, 20000);
        })
    });
}

criterion_group!(benches, bench_query);
criterion_main!(benches);
