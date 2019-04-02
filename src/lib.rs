mod image;
mod ray;
pub mod vec3;

use crate::image::Image;
use crate::ray::{Hit, Hitable, Ray, Sphere};
use crate::vec3::Vec3;

pub const ORIGIN: Vec3 = Vec3(0.0, 0.0, 0.0);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

#[allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn gen_image(camera: Camera, pixel_density: f64) -> Image {
    let n_columns_x: f64 = pixel_density * camera.horizontal.length();
    let n_rows_y: f64 = pixel_density * camera.vertical.length();
    let mut pixel_colors: Vec<Vec3> = Vec::with_capacity((n_columns_x * n_rows_y) as usize);
    let max_channel_value: f64 = 255.0;
    for y in (0..(n_rows_y as i32)).rev() {
        let v = (y as f64) / (n_rows_y - 1.0);
        for x in 0..(n_columns_x as i32) {
            let u = (x as f64) / (n_columns_x - 1.0);
            let ray = Ray {
                origin: ORIGIN,
                direction: camera.lower_left_corner
                    + (u * camera.horizontal)
                    + (v * camera.vertical),
            };
            let color = max_channel_value * ray_color(ray);
            pixel_colors.push(color);
        }
    }
    Image {
        columns: n_columns_x as i32,
        rows: n_rows_y as i32,
        max_channel_value,
        pixel_colors,
    }
}

fn scale_value_to_range(
    range_in_min: f64,
    range_in_max: f64,
    range_out_min: f64,
    range_out_max: f64,
    in_value: f64,
) -> f64 {
    let range_in = range_in_max - range_in_min;
    let range_out = range_out_max - range_out_min;
    let scale = range_out / range_in;
    let post_scale_shift = range_out_min - (range_in_min * scale);
    (in_value * scale) + post_scale_shift
}

fn ray_color(r: Ray) -> Vec3 {
    let objects: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3(0.0, -1000.5, -1.0),
            radius: 1000.0,
        }),
    ];
    let sky_color_0 = Vec3(1.0, 1.0, 1.0);
    let sky_color_1 = Vec3(0.5, 0.7, 1.0);
    if let Hit::Hit { normal, .. } = objects.hit(r) {
        return Vec3(
            scale_value_to_range(-1.0, 1.0, 0.0, 1.0, normal.0),
            scale_value_to_range(-1.0, 1.0, 0.0, 1.0, normal.1),
            scale_value_to_range(-1.0, 1.0, 0.0, 1.0, normal.2),
        );
    }
    let unit_direction = r.direction.normalized();
    let y = scale_value_to_range(-1.0, 1.0, 0.0, 1.0, unit_direction.1);
    ((1.0 - y) * sky_color_0) + (y * sky_color_1)
}

#[test]
fn test_scale_value_to_range() {
    assert_eq!(scale_value_to_range(-1.0, 1.0, 0.0, 1.0, -1.0), 0.0);
    assert_eq!(scale_value_to_range(-1.0, 1.0, 0.0, 1.0, 0.0), 0.5);
    assert_eq!(scale_value_to_range(-1.0, 1.0, 0.0, 1.0, 1.0), 1.0);
    assert_eq!(scale_value_to_range(-7.0, -6.0, 2.0, 4.0, -7.0), 2.0);
    assert_eq!(scale_value_to_range(-7.0, -6.0, 2.0, 4.0, -6.5), 3.0);
    assert_eq!(scale_value_to_range(-7.0, -6.0, 2.0, 4.0, -6.0), 4.0);
}

#[test]
fn test_gen_image() {
    let c = Camera {
        origin: ORIGIN,
        lower_left_corner: Vec3(-2.0, -1.0, -1.0),
        horizontal: Vec3(4.0, 0.0, 0.0),
        vertical: Vec3(0.0, 2.0, 0.0),
    };
    let image = gen_image(c, 10.0);
    assert_eq!(image.columns, 40);
    assert_eq!(image.rows, 20);
    assert_eq!(image.pixel_colors.len(), 800);
}
