#![warn(
    warnings,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rustdoc,
    unused
)]

use raytrace::gen_image;
use std::io;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    gen_image(400, 200).write_to(&mut stdout)
}
