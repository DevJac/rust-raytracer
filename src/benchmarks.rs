use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use raytrace::vec3::Vec3;
use raytrace::{gen_image, Camera, ORIGIN, UP};

fn bench_gen_image(criterion: &mut Criterion) {
    let c = Camera {
        up: UP,
        look_from: ORIGIN,
        look_to: Vec3(0.0, 0.0, -1.0),
        aspec_ratio: 2.0,
        vertical_fov: 45.0,
    };
    criterion.bench_function("gen_image", move |b: &mut Bencher| {
        b.iter(|| gen_image(c, 100.0, 5))
    });
}

criterion_group!(benches, bench_gen_image);
criterion_main!(benches);
