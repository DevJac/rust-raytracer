mod image;
pub mod ray;
pub mod vec3;

use crate::image::Image;
use crate::ray::{Hitable, Ray};
use crate::vec3::Vec3;
use random::Source as _;
use std::time::{Duration, SystemTime};

pub const ORIGIN: Vec3 = Vec3(0.0, 0.0, 0.0);
pub const UP: Vec3 = Vec3(0.0, 1.0, 0.0);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub up: Vec3,
    pub look_from: Vec3,
    pub look_to: Vec3,
    pub aspec_ratio: f64,
    pub vertical_fov: f64,
}

impl Camera {
    fn lower_left_corner(&self) -> Vec3 {
        self.look_from + (self.look_to.normalized() * self.vertical_fov.to_radians().tan().powi(-1))
            - self.horizontal()
            - self.vertical()
    }

    fn horizontal(&self) -> Vec3 {
        self.look_to.cross(self.up).normalized() * self.aspec_ratio
    }

    fn vertical(&self) -> Vec3 {
        self.horizontal().cross(self.look_to).normalized()
    }
}

#[allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn gen_image(
    world: &[Box<dyn Hitable>],
    camera: Camera,
    horizontal_pixels: f64,
    aa_rays: i32,
) -> Image {
    let n_columns_x: f64 = horizontal_pixels.round();
    let n_rows_y: f64 = (horizontal_pixels / camera.aspec_ratio).round();
    let mut random_source = random::default();
    let mut pixel_colors: Vec<Vec3> = Vec::with_capacity((n_columns_x * n_rows_y) as usize);
    let max_channel_value: f64 = 255.0;
    let mut last_print = SystemTime::now();
    for y in (0..(n_rows_y as i32)).rev() {
        let now = SystemTime::now();
        let ds = now.duration_since(last_print);
        if ds.is_err() || ds.unwrap() > Duration::from_secs(30) {
            eprintln!(
                "Rendering: {:.0}% complete",
                (n_rows_y - (y as f64)) / n_rows_y * 100.0,
            );
            last_print = now;
        }
        for x in 0..(n_columns_x as i32) {
            let mut average_color = Vec3(0.0, 0.0, 0.0);
            for aa_ray_i in 1..=aa_rays {
                let v = ((y as f64) + random_source.read::<f64>() - 0.5) / (n_rows_y - 1.0);
                let u = ((x as f64) + random_source.read::<f64>() - 0.5) / (n_columns_x - 1.0);
                let ray = Ray {
                    origin: camera.look_from,
                    direction: camera.lower_left_corner()
                        + (u * 2.0 * camera.horizontal())
                        + (v * 2.0 * camera.vertical()),
                };
                let aa_ray_color = ray.color(world);
                average_color += (aa_ray_color - average_color) / (aa_ray_i as f64);
            }
            pixel_colors.push(gamma_correct(average_color) * max_channel_value);
        }
    }
    Image {
        columns: n_columns_x as i32,
        rows: n_rows_y as i32,
        max_channel_value,
        pixel_colors,
    }
}

fn gamma_correct(c: Vec3) -> Vec3 {
    Vec3(c.0.sqrt(), c.1.sqrt(), c.2.sqrt())
}

#[cfg(test)]
use crate::ray::{Sphere, StandardMaterial};

#[test]
fn test_gen_image() {
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
        look_from: ORIGIN,
        look_to: Vec3(0.0, 0.0, -1.0),
        aspec_ratio: 2.0,
        vertical_fov: 45.0,
    };
    let image = gen_image(&world, c, 10.0, 8);
    assert_eq!(image.columns, 10);
    assert_eq!(image.rows, 5);
    assert_eq!(image.pixel_colors.len(), 50);
}
