use super::point::Point3;
use super::ray::Ray;
use super::vector::Vector3;
use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3 {
    rows: [[f32; 3]; 3],
}

impl Neg for Matrix3 {
    type Output = Matrix3;

    fn neg(self) -> Matrix3 {
        let mut result = Matrix3 {
            rows: [[0_f32; 3]; 3],
        };

        for col in 0..3 {
            for row in 0..3 {
                result.rows[row][col] = -self.rows[row][col];
            }
        }

        result
    }
}

impl Div<f32> for Matrix3 {
    type Output = Matrix3;

    fn div(self, other: f32) -> Matrix3 {
        let mut result = Matrix3 {
            rows: [[0_f32; 3]; 3],
        };

        for col in 0..3 {
            for row in 0..3 {
                result.rows[row][col] = self.rows[row][col] / other;
            }
        }

        result
    }
}

impl DivAssign<f32> for Matrix3 {
    fn div_assign(&mut self, other: f32) {
        for col in 0..3 {
            for row in 0..3 {
                self.rows[row][col] = self.rows[row][col] / other;
            }
        }
    }
}

impl Mul<f32> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, other: f32) -> Matrix3 {
        let mut result = Matrix3 {
            rows: [[0_f32; 3]; 3],
        };

        for col in 0..3 {
            for row in 0..3 {
                result.rows[row][col] = self.rows[row][col] * other;
            }
        }

        result
    }
}

impl MulAssign<f32> for Matrix3 {
    fn mul_assign(&mut self, other: f32) {
        for col in 0..3 {
            for row in 0..3 {
                self.rows[row][col] = self.rows[row][col] * other;
            }
        }
    }
}

impl Mul for Matrix3 {
    type Output = Matrix3;

    fn mul(self, other: Matrix3) -> Matrix3 {
        let mut result = Matrix3 {
            rows: [[0_f32; 3]; 3],
        };

        for col in 0..3 {
            for row in 0..3 {
                let mut sum = 0_f32;
                for i in 0..3 {
                    sum += self.rows[row][i] * other.rows[i][col];
                }

                result.rows[row][col] = sum;
            }
        }

        result
    }
}

impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.rows[0][0] * other.x + self.rows[0][1] * other.y + self.rows[0][2] * other.z,
            y: self.rows[1][0] * other.x + self.rows[1][1] * other.y + self.rows[1][2] * other.z,
            z: self.rows[2][0] * other.x + self.rows[2][1] * other.y + self.rows[2][2] * other.z,
        }
    }
}

impl Mul<Point3> for Matrix3 {
    type Output = Point3;

    fn mul(self, other: Point3) -> Point3 {
        Point3 {
            x: self.rows[0][0] * other.x + self.rows[0][1] * other.y + self.rows[0][2] * other.z,
            y: self.rows[1][0] * other.x + self.rows[1][1] * other.y + self.rows[1][2] * other.z,
            z: self.rows[2][0] * other.x + self.rows[2][1] * other.y + self.rows[2][2] * other.z,
        }
    }
}

impl Mul<Ray> for Matrix3 {
    type Output = Ray;

    fn mul(self, other: Ray) -> Ray {
        Ray::new(self * other.origin, (self * other.direction).normalised())
    }
}

impl Matrix3 {
    pub fn identity() -> Matrix3 {
        Matrix3::scale(1_f32)
    }

    pub fn scale(scale: f32) -> Matrix3 {
        let mut result = Matrix3 {
            rows: [[0_f32; 3]; 3],
        };

        result.rows[0][0] = scale;
        result.rows[1][1] = scale;
        result.rows[2][2] = scale;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix3_identity_mulitplication() {
        let id = Matrix3::identity();
        assert_eq!(id * id, id);
    }

    #[test]
    fn matrix3_scale_mulitplication() {
        let matrix_two = Matrix3::scale(2_f32);
        assert_eq!(matrix_two * matrix_two, Matrix3::scale(4_f32));
    }

    #[test]
    fn matrix3_scaler_mulitplication() {
        let matrix_two = Matrix3::scale(2_f32);
        assert_eq!(matrix_two * 2_f32, Matrix3::scale(4_f32));
    }

    #[test]
    fn matrix3_vector3_identity_multiplication() {
        let vec = Vector3::unit_x();
        let identity = Matrix3::identity();

        assert_eq!(identity * vec, vec);

        assert_eq!(identity * 2_f32 * vec, vec * 2_f32);
    }

}
