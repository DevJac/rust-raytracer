use crate::vec3::Vec3;
use std::io;

pub struct Image {
    pub columns: i32,
    pub rows: i32,
    pub max_channel_value: f64,
    pub pixel_colors: Vec<Vec3>,
}

impl Image {
    pub fn write_to(&self, writable: &mut impl io::Write) -> io::Result<()> {
        write!(
            writable,
            "P3\n{} {}\n{:.0}\n",
            self.columns, self.rows, self.max_channel_value
        )?;
        for color in &self.pixel_colors {
            writeln!(writable, "{}", color.as_ppm_pixel())?;
        }
        Ok(())
    }
}
