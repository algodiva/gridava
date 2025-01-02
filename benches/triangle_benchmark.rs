use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use gridava::triangle::coordinate::Triangle;

fn tri_line(c: &mut Criterion) {
    let mut group = c.benchmark_group("tri_line");
    let a = Triangle::new(0, 1, 0);
    for b in [
        Triangle::new(1, 2, -1),
        Triangle::new(1, 2, -2),
        Triangle::new(2, 2, -2),
        Triangle::new(2, 2, -3),
        Triangle::new(3, 2, -3),
        Triangle::new(3, 2, -4),
        Triangle::new(4, 2, -4),
        Triangle::new(4, 2, -5),
        Triangle::new(5, 2, -5),
        Triangle::new(5, 2, -6),
        Triangle::new(6, 2, -6),
        Triangle::new(6, 2, -7),
        Triangle::new(7, 2, -7),
        Triangle::new(7, 2, -8),
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
        Triangle::new(0, 2, 0),
        Triangle::new(0, 2, -1),
        Triangle::new(0, 3, -1),
        Triangle::new(0, 3, -2),
        Triangle::new(0, 4, -2),
        Triangle::new(0, 4, -3),
        Triangle::new(0, 5, -3),
        Triangle::new(0, 5, -4),
        Triangle::new(0, 6, -4),
        Triangle::new(0, 6, -5),
        Triangle::new(0, 7, -5),
        Triangle::new(0, 7, -6),
        Triangle::new(0, 8, -6),
        Triangle::new(0, 8, -7),
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
