mod image;
mod ray;
pub mod vec3;

use crate::image::Image;
use crate::ray::{Hit, Hitable, Ray, Sphere};
use crate::vec3::Vec3;
use random::Source as _;

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
pub fn gen_image(camera: Camera, horizontal_pixels: f64, aa_rays: i32) -> Image {
    let n_columns_x: f64 = horizontal_pixels.round();
    let n_rows_y: f64 =
        (horizontal_pixels * (camera.vertical.length() / camera.horizontal.length())).round();
    let mut random_source = random::default();
    let mut pixel_colors: Vec<Vec3> = Vec::with_capacity((n_columns_x * n_rows_y) as usize);
    let max_channel_value: f64 = 255.0;
    for y in (0..(n_rows_y as i32)).rev() {
        for x in 0..(n_columns_x as i32) {
            let mut average_color = Vec3(0.0, 0.0, 0.0);
            for aa_ray_i in 1..=aa_rays {
                let v = ((y as f64) + random_source.read::<f64>() - 0.5) / (n_rows_y - 1.0);
                let u = ((x as f64) + random_source.read::<f64>() - 0.5) / (n_columns_x - 1.0);
                let ray = Ray {
                    origin: ORIGIN,
                    direction: camera.lower_left_corner
                        + (u * camera.horizontal)
                        + (v * camera.vertical),
                };
                let aa_ray_color = ray_color(ray);
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
    if let Hit::Hit {
        point: hit_point,
        normal: hit_normal,
        ..
    } = objects.hit(r)
    {
        return 0.5
            * ray_color(Ray {
                origin: hit_point,
                direction: hit_normal + random_point_in_unit_sphere(),
            });
    }
    let unit_direction = r.direction.normalized();
    let y = scale_value_to_range(-1.0, 1.0, 0.0, 1.0, unit_direction.1);
    ((1.0 - y) * sky_color_0) + (y * sky_color_1)
}

fn random_point_in_unit_sphere() -> Vec3 {
    let mut random_source = random::default();
    loop {
        let p =
            (2.0 * Vec3(
                random_source.read(),
                random_source.read(),
                random_source.read(),
            )) - Vec3(1.0, 1.0, 1.0);
        if p.length() <= 1.0 {
            return p;
        }
    }
}

#[test]
fn test_random_point_in_unit_sphere() {
    let a = random_point_in_unit_sphere();
    let b = random_point_in_unit_sphere();
    let d0 = (a.0 - b.0).abs();
    let d1 = (a.1 - b.1).abs();
    let d2 = (a.2 - b.2).abs();
    assert!(d0 > 0.001 || d1 > 0.001 || d2 > 0.001);
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
    let image = gen_image(c, 10.0, 8);
    assert_eq!(image.columns, 10);
    assert_eq!(image.rows, 5);
    assert_eq!(image.pixel_colors.len(), 50);
}
