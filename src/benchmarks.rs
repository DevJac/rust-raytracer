use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use raytrace::ray::{Hitable, Sphere, StandardMaterial};
use raytrace::vec3::Vec3;
use raytrace::{gen_image, Camera, UP};

fn bench_gen_image(criterion: &mut Criterion) {
    let world: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
            material: StandardMaterial {
                reflection: 0.98,
                color: Vec3(1.0, 0.2, 0.2),
                albedo: 0.6,
            },
        }),
        Box::new(Sphere {
            center: Vec3(0.0, -1000.5, -1.0),
            radius: 1000.0,
            material: StandardMaterial {
                reflection: 0.0,
                color: Vec3(0.3, 0.6, 0.0),
                albedo: 0.4,
            },
        }),
        Box::new(Sphere {
            center: Vec3(0.9, -0.3, -0.9),
            radius: 0.2,
            material: StandardMaterial {
                reflection: 0.0,
                color: Vec3(0.2, 0.2, 1.0),
                albedo: 0.6,
            },
        }),
        Box::new(Sphere {
            center: Vec3(-1.0, -0.1, -0.9),
            radius: 0.4,
            material: StandardMaterial {
                reflection: 1.0,
                color: Vec3(1.0, 1.0, 0.6),
                albedo: 0.8,
            },
        }),
    ];
    let c = Camera {
        up: UP,
        look_from: Vec3(0.0, 0.0, 1.0),
        look_to: Vec3(0.0, 0.0, -1.0),
        aspec_ratio: 2.0,
        vertical_fov: 15.0,
    };
    criterion.bench_function("gen_image", move |b: &mut Bencher| {
        b.iter(|| gen_image(&world, c, 100.0, 5))
    });
}

criterion_group!(benches, bench_gen_image);
criterion_main!(benches);
