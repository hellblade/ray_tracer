use super::point::*;
use super::vector::*;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }
}
