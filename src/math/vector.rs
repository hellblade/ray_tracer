use std::ops::*;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f32) -> Vector3 {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vector3 {
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalise(&mut self) {
        let len = self.length();

        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn dot(self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: 0_f32,
        }
    }

    pub fn unit_x() -> Vector3 {
        Vector3 {
            x: 1_f32,
            y: 0_f32,
            z: 0_f32,
        }
    }

    pub fn unit_y() -> Vector3 {
        Vector3 {
            x: 0_f32,
            y: 1_f32,
            z: 0_f32,
        }
    }

    pub fn unit_z() -> Vector3 {
        Vector3 {
            x: 0_f32,
            y: 0_f32,
            z: 1_f32,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector3_length() {
        let vec = Vector3 {
            x: 1_f32,
            y: 0_f32,
            z: 0_f32,
        };
        assert_eq!(vec.length(), 1_f32);
    }

    #[test]
    fn vector3_normalise() {
        let mut vec = Vector3 {
            x: 3f32,
            y: 3f32,
            z: 3f32,
        };
        vec.normalise();
        //assert_eq!(vec, 1f32);
    }

    #[test]
    fn vector3_dot_matches_length() {
        let vec = Vector3 {
            x: 1f32,
            y: 1f32,
            z: 1f32,
        };
        assert_eq!(vec.dot(vec), vec.length_squared());
    }
}
