use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use raytrace::vec3::Vec3;
use raytrace::{gen_image, Camera, ORIGIN};

fn bench_gen_image(criterion: &mut Criterion) {
    let c = Camera {
        origin: ORIGIN,
        lower_left_corner: Vec3(-2.0, -1.0, -1.0),
        horizontal: Vec3(4.0, 0.0, 0.0),
        vertical: Vec3(0.0, 2.0, 0.0),
    };
    let image_sizes: Vec<f64> = vec![100.0, 200.0, 400.0];
    criterion.bench_function_over_inputs(
        "gen_image",
        move |b: &mut Bencher, size: &f64| b.iter(|| gen_image(c, *size, 8)),
        image_sizes,
    );
}

criterion_group!(benches, bench_gen_image);
criterion_main!(benches);
