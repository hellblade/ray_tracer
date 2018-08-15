extern crate float_cmp;

use super::point::*;
use super::ray::*;
use super::vector::*;
use math::sphere::float_cmp::ApproxEqUlps;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Sphere {
    pub centre: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn intersect(&self, ray: &Ray) -> (bool, f32) {
        let v = ray.origin - self.centre;

        let b = v.dot(ray.direction);
        let c = v.dot(v) - (self.radius * self.radius);

        if c > 0_f32 && b > 0_f32 {
            return (false, 0_f32);
        }

        let discriminant = b * b - c;

        if discriminant < 0_f32 {
            return (false, 0_f32);
        }

        let distance = (-b - discriminant.sqrt()).max(0_f32);

        (true, distance)
    }

    pub fn new(centre: Point3, radius: f32) -> Sphere {
        Sphere { centre, radius }
    }

    /// Gets the Normal at a given point on the sphere
    ///
    /// Expects that the point is on the sphere - doesn't normalise
    pub fn get_normal(&self, point: Point3) -> Vector3 {
        (point - self.centre) / self.radius
    }
}
