use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at_t(&self, t: f64) -> Vec3 {
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

pub enum Hit {
    NoHit,
    Hit { t: f64, point: Vec3, normal: Vec3 },
}

pub trait Hitable {
    fn hit(&self, r: Ray) -> Hit;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray) -> Hit {
        // We're trying to solve: dot((A + t*B - C), (A + t*B - C)) = R*R
        // Where A is the ray's origin, B is the ray's direction,
        // C is the center of the sphere, and R is the radius of the sphere.
        // The above formula can be rewritten as: t*t*dot(B,B) + 2*t*dot(A-C,B) + dot(A-C,A-C) - R*R = 0
        // A, B, and C are known constants. We can use the quadratic formula to solve the equation.
        // The quadratic formula solves an equation of the form: ax^2 + bx + c
        // Importantly, we need to calculate the discriminant: b*b - 4*a*c
        // If the discriminant is greater than zero, we are intersecting the sphere's surface at 2 points (front and back).
        let oc = r.origin - self.center; // This is A - C
        let a = r.direction.dot(r.direction); // This is dot(B,B) of t*t*dot(B,B)
        let b = 2.0 * oc.dot(r.direction); // This is 2*dot(A-C,B) of 2*t*dot(A-C,B); we are working with the coefficients of t, since we are solving for t.
        let c = oc.dot(oc) - self.radius.powi(2); // This is dot(A-C,A-C) - R*R; these are constants, there is no t in this term.
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            Hit::NoHit
        } else {
            let hit_t = (-b - discriminant.sqrt()) / (2.0 * a);
            let hit_point = r.point_at_t(hit_t);
            Hit::Hit {
                t: hit_t,
                point: hit_point,
                normal: (hit_point - self.center) / self.radius,
            }
        }
    }
}
