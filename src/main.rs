#![warn(
    warnings,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rustdoc,
    unused
)]

mod ray;
mod vec3;

use crate::ray::Ray;
use crate::vec3::Vec3;
use std::io;
use std::io::Write as _;

const ORIGIN: Vec3 = Vec3(0.0, 0.0, 0.0);

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    write_image(&mut stdout)
}

#[allow(clippy::cast_lossless)]
fn write_image(stdout: &mut io::Stdout) -> io::Result<()> {
    let n_columns_x: i32 = 200;
    let n_rows_y: i32 = 100;
    let max_channel_value: f64 = 255.0;
    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    write!(
        stdout,
        "P3\n{} {}\n{:.0}\n",
        n_columns_x, n_rows_y, max_channel_value
    )?;
    for y in (0..n_rows_y).rev() {
        let v = (y as f64) / ((n_rows_y as f64) - 1.0);
        for x in 0..n_columns_x {
            let u = (x as f64) / ((n_columns_x as f64) - 1.0);
            let ray = Ray {
                origin: ORIGIN,
                direction: lower_left_corner + (u * horizontal) + (v * vertical),
            };
            let color = max_channel_value * ray_color(&ray);
            writeln!(stdout, "{}", color.as_ppm_pixel())?;
        }
    }
    Ok(())
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

fn ray_color(r: &Ray) -> Vec3 {
    let color_0 = Vec3(1.0, 1.0, 1.0);
    let color_1 = Vec3(0.5, 0.7, 1.0);
    let unit_direction = r.direction.normalized();
    let y = scale_value_to_range(-1.0, 1.0, 0.0, 1.0, unit_direction.1);
    ((1.0 - y) * color_0) + (y * color_1)
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
