use super::vector::*;
use std::ops::*;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Sub for Point3 {
    type Output = Vector3;

    fn sub(self, other: Point3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
    }

    pub fn zero() -> Point3 {
        Point3 {
            x: 0_f32,
            y: 0_f32,
            z: 0_f32,
        }
    }
}
