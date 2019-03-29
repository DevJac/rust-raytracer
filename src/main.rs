#![warn(
    warnings,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rustdoc,
    unused
)]

mod vec3;

use crate::vec3::Vec3;
use std::io;
use std::io::Write as _;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    write_image(&mut stdout)
}

#[allow(clippy::cast_lossless)]
fn write_image(stdout: &mut io::Stdout) -> io::Result<()> {
    let n_columns_x: i32 = 200;
    let n_rows_y: i32 = 100;
    let max_channel_value: f64 = 255.0;
    write!(
        stdout,
        "P3\n{} {}\n{:.0}\n",
        n_columns_x, n_rows_y, max_channel_value
    )?;
    for y in 0..n_rows_y {
        for x in 0..n_columns_x {
            let color = max_channel_value
                * Vec3(
                    (x as f64) / ((n_columns_x as f64) - 1.0),
                    (y as f64) / ((n_rows_y as f64) - 1.0),
                    0.2,
                );
            writeln!(stdout, "{}", color.as_ppm_pixel())?;
        }
    }
    Ok(())
}
