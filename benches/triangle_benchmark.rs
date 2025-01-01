use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use gridava::triangle;
use gridava::triangle::coordinate::Triangle;

fn tri_line(c: &mut Criterion) {
    let mut group = c.benchmark_group("tri_line");
    let a = triangle!(0, 1, 0);
    for b in [
        triangle!(1, 2, -1),
        triangle!(1, 2, -2),
        triangle!(2, 2, -2),
        triangle!(2, 2, -3),
        triangle!(3, 2, -3),
        triangle!(3, 2, -4),
        triangle!(4, 2, -4),
        triangle!(4, 2, -5),
        triangle!(5, 2, -5),
        triangle!(5, 2, -6),
        triangle!(6, 2, -6),
        triangle!(6, 2, -7),
        triangle!(7, 2, -7),
        triangle!(7, 2, -8),
    ]
    .iter()
    {
        group.throughput(criterion::Throughput::Elements(a.distance(*b) as u64));
        group.bench_with_input(
            BenchmarkId::new("smooth", a.distance(*b)),
            b,
            |bench, &b| {
                bench.iter(|| a.line(b));
            },
        );
    }

    for b in [
        triangle!(0, 2, 0),
        triangle!(0, 2, -1),
        triangle!(0, 3, -1),
        triangle!(0, 3, -2),
        triangle!(0, 4, -2),
        triangle!(0, 4, -3),
        triangle!(0, 5, -3),
        triangle!(0, 5, -4),
        triangle!(0, 6, -4),
        triangle!(0, 6, -5),
        triangle!(0, 7, -5),
        triangle!(0, 7, -6),
        triangle!(0, 8, -6),
        triangle!(0, 8, -7),
    ]
    .iter()
    {
        group.throughput(criterion::Throughput::Elements(a.distance(*b) as u64));
        group.bench_with_input(
            BenchmarkId::new("along_axis", a.distance(*b)),
            b,
            |bench, &b| {
                bench.iter(|| a.line(b));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, tri_line);
criterion_main!(benches);
