mod image;
mod ray;
pub mod vec3;

use crate::image::Image;
use crate::ray::Ray;
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
                let aa_ray_color = ray.color();
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
