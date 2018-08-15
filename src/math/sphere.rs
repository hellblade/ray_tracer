extern crate float_cmp;

use super::point::*;
use super::ray::*;
use math::sphere::float_cmp::ApproxEqUlps;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Sphere {
    pub centre: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn intersect(self, ray: Ray) -> (bool, f32) {
        let ray_to_sphere = ray.origin - self.centre;
        let ray_to_sphere_length_squared = ray_to_sphere.length_squared();

        let direction_dot = ray.direction.dot(ray_to_sphere);
        let direction_dot_squared = direction_dot * direction_dot;

        let diffs =
            direction_dot_squared - ray_to_sphere_length_squared + self.radius * self.radius;

        if diffs.approx_eq_ulps(&0_f32, 2) {
            // Only one solution

            return (true, -direction_dot);
        } else if diffs < 0_f32 {
            // No solutions
            return (false, 0_f32);
        }

        // Two solutions, so find the minimum one

        (true, -direction_dot - diffs.sqrt())
    }

    pub fn new(centre: Point3, radius: f32) -> Sphere {
        Sphere { centre, radius }
    }
}
