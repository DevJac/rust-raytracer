use criterion::{criterion_group, criterion_main, Bencher, Criterion};

fn bench_gen_image(c: &mut Criterion) {
    let image_sizes: Vec<i32> = vec![100, 200, 400];
    c.bench_function_over_inputs(
        "gen_image",
        |b: &mut Bencher, size: &i32| b.iter(|| raytrace::gen_image(*size, size / 2)),
        image_sizes,
    );
}

criterion_group!(benches, bench_gen_image);
criterion_main!(benches);
