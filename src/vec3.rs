use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn length(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    // Tarpaulin doesn't detect coverage of this function, but it is covered.
    #[cfg_attr(tarpaulin, skip)]
    pub fn dot(&self, rhs: Self) -> f64 {
        (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2)
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self(
            (self.1 * rhs.2) - (self.2 * rhs.1),
            -((self.0 * rhs.2) - (self.2 * rhs.0)),
            (self.0 * rhs.1) - (self.1 * rhs.0),
        )
    }

    pub fn as_ppm_pixel(&self) -> String {
        format!("{:.0} {:.0} {:.0}", self.0, self.1, self.2)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[test]
fn test_length() {
    assert_approx_eq!(Vec3(2.0, 3.0, 4.5).length(), 5.766281);
}

#[test]
fn test_normalized() {
    let normalized_vector = Vec3(1.0, 2.0, 3.0).normalized();
    assert_approx_eq!(normalized_vector.0, 0.267261);
    assert_approx_eq!(normalized_vector.1, 0.534523);
    assert_approx_eq!(normalized_vector.2, 0.801784);
}

#[test]
fn test_dot() {
    assert_eq!(Vec3(1.0, 2.0, 3.0).dot(Vec3(1.0, 3.3, 0.0)), 7.6);
}

#[test]
fn test_cross() {
    assert_eq!(
        Vec3(1.0, 2.0, 3.0).cross(Vec3(0.0, 2.0, 3.0)),
        Vec3(0.0, -3.0, 2.0)
    )
}

#[test]
fn test_as_ppm_pixel() {
    // Watch out for the rounding of 2.5.
    // Remember, 2.5 cannot be exactly represented as an f64.
    assert_eq!(Vec3(0.4, 1.6, 2.5).as_ppm_pixel(), "0 2 2")
}

#[test]
fn test_math() {
    let a = Vec3(1.0, 2.0, 3.0);
    let b = Vec3(2.0, 2.0, 2.0);
    assert_eq!(a + b, Vec3(3.0, 4.0, 5.0));
    assert_eq!(a - b, Vec3(-1.0, 0.0, 1.0));
    assert_eq!(a * 2.0, Vec3(2.0, 4.0, 6.0));
    assert_eq!(2.0 * a, Vec3(2.0, 4.0, 6.0));
    assert_eq!(a * b, Vec3(2.0, 4.0, 6.0));
    assert_eq!(a / 2.0, Vec3(0.5, 1.0, 1.5));
    assert_eq!(a / b, Vec3(0.5, 1.0, 1.5));
    let mut c = a.clone();
    c *= 2.0;
    assert_eq!(a * 2.0, c);
    c /= 2.0;
    assert_eq!(a, c);
    let mut d = a.clone();
    d *= b;
    assert_eq!(a * 2.0, d);
    d /= b;
    assert_eq!(a, d);
}
