use crate::vec3::Vec3;
use random::Source as _;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at_t(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }

    pub fn color(&self) -> Vec3 {
        let objects: Vec<Box<dyn Hitable>> = vec![
            Box::new(Sphere {
                center: Vec3(0.0, 0.0, -1.0),
                radius: 0.5,
                material: StandardMaterial {
                    reflection: 0.0,
                    color: Vec3(1.0, 1.0, 1.0),
                    albedo: 0.5,
                },
            }),
            Box::new(Sphere {
                center: Vec3(0.0, -1000.5, -1.0),
                radius: 1000.0,
                material: StandardMaterial {
                    reflection: 0.0,
                    color: Vec3(1.0, 1.0, 1.0),
                    albedo: 0.5,
                },
            }),
        ];
        let sky_color_0 = Vec3(1.0, 1.0, 1.0);
        let sky_color_1 = Vec3(0.5, 0.7, 1.0);
        if let Hit::Hit {
            point: hit_point,
            normal: hit_normal,
            material: hit_material,
            ..
        } = objects.hit(*self)
        {
            return hit_material.attenuate(
                hit_material
                    .scatter(self.direction, hit_point, hit_normal)
                    .color(),
            );
        }
        let unit_direction = self.direction.normalized();
        let y = scale_value_to_range(-1.0, 1.0, 0.0, 1.0, unit_direction.1);
        ((1.0 - y) * sky_color_0) + (y * sky_color_1)
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hit {
    NoHit,
    Hit {
        t: f64,
        point: Vec3,
        normal: Vec3,
        material: StandardMaterial,
    },
}

pub trait Hitable {
    fn hit(&self, r: Ray) -> Hit;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: StandardMaterial,
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
            if hit_t < 0.0 {
                Hit::NoHit
            } else {
                Hit::Hit {
                    t: hit_t,
                    point: hit_point,
                    normal: (hit_point - self.center) / self.radius,
                    material: self.material,
                }
            }
        }
    }
}

impl Hitable for Vec<Box<dyn Hitable>> {
    fn hit(&self, r: Ray) -> Hit {
        let mut nearest_hit = Hit::NoHit;
        for hitable in self {
            let hitable = hitable.as_ref();
            let hit = hitable.hit(r);
            match (nearest_hit, hit) {
                (Hit::NoHit, Hit::Hit { .. }) => nearest_hit = hit,
                (Hit::Hit { t, .. }, Hit::Hit { t: t_new, .. }) if t_new < t => nearest_hit = hit,
                _ => (),
            }
        }
        nearest_hit
    }
}

trait Material {
    fn scatter(
        &self,
        incoming_ray_direction: Vec3,
        surface_point: Vec3,
        surface_normal: Vec3,
    ) -> Ray;
    fn attenuate(&self, incoming_ray_color: Vec3) -> Vec3;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StandardMaterial {
    pub reflection: f64,
    pub color: Vec3,
    pub albedo: f64,
}

impl Material for StandardMaterial {
    fn scatter(
        &self,
        incoming_ray_direction: Vec3,
        surface_point: Vec3,
        surface_normal: Vec3,
    ) -> Ray {
        let reflected_ray_direction = incoming_ray_direction
            - 2.0 * incoming_ray_direction.dot(surface_normal) * surface_normal;
        let diffuse_scattered_ray_direction = random_point_in_unit_sphere() + surface_normal;
        let combined_direction = self.reflection * reflected_ray_direction
            + (1.0 - self.reflection) * diffuse_scattered_ray_direction;
        Ray {
            origin: surface_point,
            direction: combined_direction.normalized(),
        }
    }

    fn attenuate(&self, incoming_ray_color: Vec3) -> Vec3 {
        self.albedo * self.color * incoming_ray_color
    }
}

fn random_point_in_unit_sphere() -> Vec3 {
    let mut random_source = random::default();
    loop {
        let p =
            (2.0 * Vec3(
                random_source.read(),
                random_source.read(),
                random_source.read(),
            )) - Vec3(1.0, 1.0, 1.0);
        if p.length() <= 1.0 {
            return p;
        }
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
fn test_random_point_in_unit_sphere() {
    let a = random_point_in_unit_sphere();
    let b = random_point_in_unit_sphere();
    let d0 = (a.0 - b.0).abs();
    let d1 = (a.1 - b.1).abs();
    let d2 = (a.2 - b.2).abs();
    assert!(d0 > 0.001 || d1 > 0.001 || d2 > 0.001);
}
