mod image;
mod ray;
mod vec3;

use crate::image::Image;
use crate::ray::Ray;
use crate::vec3::Vec3;

const ORIGIN: Vec3 = Vec3(0.0, 0.0, 0.0);

#[allow(clippy::cast_lossless, clippy::cast_sign_loss)]
pub fn gen_image(n_columns_x: i32, n_rows_y: i32) -> Image {
    assert!(n_columns_x == 2 * n_rows_y, "We're assuming there's twice as many columns as rows currently, but this assumption can be removed in the future.");
    let mut pixel_colors: Vec<Vec3> = Vec::with_capacity((n_columns_x * n_rows_y) as usize);
    let max_channel_value: f64 = 255.0;
    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    for y in (0..n_rows_y).rev() {
        let v = (y as f64) / ((n_rows_y as f64) - 1.0);
        for x in 0..n_columns_x {
            let u = (x as f64) / ((n_columns_x as f64) - 1.0);
            let ray = Ray {
                origin: ORIGIN,
                direction: lower_left_corner + (u * horizontal) + (v * vertical),
            };
            let color = max_channel_value * ray_color(ray);
            pixel_colors.push(color);
        }
    }
    Image {
        columns: n_columns_x,
        rows: n_rows_y,
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
    let sky_color_0 = Vec3(1.0, 1.0, 1.0);
    let sky_color_1 = Vec3(0.5, 0.7, 1.0);
    if let Hit::Hit(hitpoint) = hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, r) {
        let blueish_hitpoint = hitpoint + Vec3(0.0, 0.0, 1.0);
        let hpn = blueish_hitpoint.normalized();
        return Vec3(
            scale_value_to_range(-1.0, 1.0, 0.0, 1.0, hpn.0),
            scale_value_to_range(-1.0, 1.0, 0.0, 1.0, hpn.1),
            scale_value_to_range(-1.0, 1.0, 0.0, 1.0, hpn.2),
        );
    }
    let unit_direction = r.direction.normalized();
    let y = scale_value_to_range(-1.0, 1.0, 0.0, 1.0, unit_direction.1);
    ((1.0 - y) * sky_color_0) + (y * sky_color_1)
}

enum Hit {
    NoHit,
    Hit(Vec3),
}

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> Hit {
    // We're trying to solve: dot((A + t*B - C), (A + t*B - C)) = R*R
    // Where A is the ray's origin, B is the ray's direction,
    // C is the center of the sphere, and R is the radius of the sphere.
    // The above formula can be rewritten as: t*t*dot(B,B) + 2*t*dot(A-C,B) + dot(A-C,A-C) - R*R = 0
    // A, B, and C are known constants. We can use the quadratic formula to solve the equation.
    // The quadratic formula solves an equation of the form: ax^2 + bx + c
    // Importantly, we need to calculate the discriminant: b*b - 4*a*c
    // If the discriminant is greater than zero, we are intersecting the sphere's surface at 2 points (front and back).
    let oc = r.origin - center; // This is A - C
    let a = r.direction.dot(r.direction); // This is dot(B,B) of t*t*dot(B,B)
    let b = 2.0 * oc.dot(r.direction); // This is 2*dot(A-C,B) of 2*t*dot(A-C,B); we are working with the coefficients of t, since we are solving for t.
    let c = oc.dot(oc) - radius.powi(2); // This is dot(A-C,A-C) - R*R; these are constants, there is no t in this term.
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        Hit::NoHit
    } else {
        let hit_t = (-b - discriminant.sqrt()) / (2.0 * a);
        Hit::Hit(r.point_at_t(hit_t))
    }
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
    let image = gen_image(10, 5);
    assert_eq!(image.columns, 10);
    assert_eq!(image.rows, 5);
    assert_eq!(image.pixel_colors.len(), 50);
}
