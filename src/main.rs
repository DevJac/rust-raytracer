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
use raytrace::{gen_image, Camera, UP};
use std::io;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let c = Camera {
        up: UP,
        look_from: Vec3(0.0, 0.0, 1.0),
        look_to: Vec3(0.0, 0.0, -1.0),
        aspec_ratio: 2.0,
        vertical_fov: 15.0,
    };
    gen_image(c, 200.0, 50).write_to(&mut stdout)
}
