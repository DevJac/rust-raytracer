#![warn(
    warnings,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rustdoc,
    unused
)]

use raytrace::vec3::Vec3;
use raytrace::{gen_image, Camera, ORIGIN};
use std::io;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let c = Camera {
        origin: ORIGIN,
        lower_left_corner: Vec3(-2.0, -1.0, -1.0),
        horizontal: Vec3(4.0, 0.0, 0.0),
        vertical: Vec3(0.0, 2.0, 0.0),
    };
    gen_image(c, 50.0).write_to(&mut stdout)
}
