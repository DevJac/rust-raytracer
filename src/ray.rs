use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at_t(self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}

#[test]
fn test_point_at_t() {
    let r = Ray {
        origin: Vec3(0.0, 0.0, 0.0),
        direction: Vec3(1.0, 2.0, 0.5),
    };
    assert_eq!(r.point_at_t(2.0), Vec3(2.0, 4.0, 1.0));
}
