use criterion::{criterion_group, criterion_main, Criterion};
use tsl::{Event, TSL};

fn mixed_workload(c: &mut Criterion) {
    c.bench_function("mixed_workload", |b| {
        b.iter(|| {
            let mut tsl = TSL::new(1000);

            for i in 0..100_000 {
                tsl.append(Event::new(i, vec![1]));

                if i % 10 == 0 {
                    let _ = tsl.range_query(i - 50, i);
                }
            }
        })
    });
}

criterion_group!(benches, mixed_workload);
criterion_main!(benches);
