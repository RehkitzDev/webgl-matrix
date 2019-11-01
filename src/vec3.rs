#[cfg(feature = "Matrix3")]
use crate::mat3::Mat3;
use crate::slice_ops::*;
#[cfg(feature = "Matrix3")]
use crate::vector::MulVectorMatrix;
use crate::vector::Vector;
use std::f32;

pub type Vec3 = [f32; 3];

impl Vector for Vec3 {
    type VectorType = Vec3;

    fn zeros() -> Self::VectorType {
        [0., 0., 0.]
    }

    fn ones() -> Self::VectorType {
        [1., 1., 1.]
    }

    fn mul(&self, rhs: &[f32]) -> Self::VectorType {
        let mut dst = *self;
        mul(&mut dst, rhs);
        dst
    }

    fn add(&self, rhs: &[f32]) -> Self::VectorType {
        let mut dst = *self;
        add(&mut dst, rhs);
        dst
    }

    fn sub(&self, rhs: &[f32]) -> Self::VectorType {
        let mut dst = *self;
        sub(&mut dst, rhs);
        dst
    }

    fn scale(&self, factor: f32) -> Self::VectorType {
        let mut dst = *self;
        scale(&mut dst, factor);
        dst
    }

    fn mag(&self) -> f32 {
        mag(self)
    }

    fn mag2(&self) -> f32 {
        mag2(self)
    }

    fn dot(&self, rhs: &[f32]) -> f32 {
        dot(self, rhs)
    }
}

#[cfg(feature = "Matrix3")]
impl MulVectorMatrix for Vec3 {
    type VectorType = Vec3;
    type MatrixType = Mat3;

    fn mul_matrix_left(&self, lhs: &Self::MatrixType) -> Self::VectorType {
        let x = self[0];
        let y = self[1];
        let z = self[2];

        [
            lhs[0] * x + lhs[1] * y + lhs[2] * z,
            lhs[3] * x + lhs[4] * y + lhs[5] * z,
            lhs[6] * x + lhs[7] * y + lhs[8] * z,
        ]
    }

    fn mul_matrix(&self, rhs: &Self::MatrixType) -> Self::VectorType {
        let x = self[0];
        let y = self[1];
        let z = self[2];

        [
            rhs[0] * x + rhs[3] * y + rhs[6] * z,
            rhs[1] * x + rhs[4] * y + rhs[7] * z,
            rhs[2] * x + rhs[5] * y + rhs[8] * z,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::almost_eq;

    #[test]
    #[cfg(feature = "Matrix3")]
    fn vec3_mul_matrix_left() {
        let a = [1., 2., 3., 4., 5., 6., 7., 8., 9.];
        let b = [11., 12., 13.];

        let c = b.mul_matrix_left(&a);
        assert_eq!(c, [74., 182., 290.]);
    }

    #[test]
    #[cfg(feature = "Matrix3")]
    fn vec3_mul_matrix() {
        let a = [1., 2., 3., 4., 5., 6., 7., 8., 9.];
        let b = [11., 12., 13.];

        let c = b.mul_matrix(&a);
        assert_eq!(c, [150., 186., 222.]);
    }

    #[test]
    fn vec3_is_immutable() {
        let b = [2., 3., 4.];
        let c = [3., 2., 3.];

        let _d = b.add(&c);
        assert_eq!(b, [2., 3., 4.]);
    }

    #[test]
    fn vec3_add() {
        let a = [1., 2., 3.];
        let b = [-1., -2., -3.];

        assert_eq!(a.add(&b), [0., 0., 0.]);
    }

    #[test]
    fn vec3_sub() {
        let a = [1., 2., 3.];
        let b = [1., 2., 3.];

        assert_eq!(a.sub(&b), [0., 0., 0.]);
    }

    #[test]
    fn vec3_mul() {
        let a = [1., 2., 3.];
        let b = [2., 3., 4.];

        assert_eq!(a.mul(&b), [2., 6., 12.]);
    }

    #[test]
    fn vec3_scale() {
        let a = [1., 2., 3.];

        assert_eq!(a.scale(3.), [3., 6., 9.]);
    }

    #[test]
    fn vec3_dot() {
        let a = [1., 2., 3.];
        let b = [2., 3., 4.];

        assert_eq!(a.dot(&b), 2. + 6. + 12.);
    }
    #[test]
    fn vec3_mag() {
        let b = [2., 3., 4.];
        assert!(almost_eq(&[b.mag()], &[5.385164807]));
    }
    #[test]
    fn vec3_mag2() {
        let b = [2., 3., 4.];
        assert!(almost_eq(&[b.mag2()], &[29.]));
    }
}
