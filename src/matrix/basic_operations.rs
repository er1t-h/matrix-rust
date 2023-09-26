//!
//! Implementation for matrix addition, substraction and multiplication by a scalar.
//!
//! # [`SafeAdd`] and [`SafeSub`] for [`Matrix`] addition and substraction
//!
//! ## Error return
//!
//! Since size is checked at runtime, both addition and substraction can fail.
//! To prevent that, [`Matrix`] implements [`SafeAdd`] and [`SafeSub`].
//! If an error occurs (dimensions differs), a [`MatrixOperationError`] is returned.
//!
//! ## Operation traits
//!
//! However, it's still practical to use the `+`, `+=`, `-` and `-=` operators.
//! Therefore, those traits are also implemented.
//! If a simple operation fails, the value of the left operand is returned, left
//! unmodified.
//!
//! If an assign operation fails, `self` is not modified either.
//!
//!
//! # [`MulAssign`] for scalar multiplication
//!
//! The trait is implemented as follow:
//! - If the scalar is given as a reference, it multiplies that reference with
//!     every element of the matrix.
//! - If the scalar is given as a value, it multiplies clones of that value with
//!     every element of the matrix.
//!
//! If you want to use custom types, you'll probably want to use the former.
//! If you want to use primitives ([`f32`], [`f64`]), you'll probably want the
//! latter.
//!
//! # Complexity
//! Each of these operations have a complexity of `O(n)`, for an `n`-element
//! matrix.
//!

mod add;
mod scalar_mul;
mod sub;

#[cfg(test)]
mod test {
    use crate::{
        complex::cpl,
        matrix::Dimensions,
        traits::{SafeAdd, SafeSub},
        Matrix,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn safe_add_assign() {
        {
            let mut lhs = Matrix::from([[1, 2, 3], [4, 5, 6]]);
            let rhs = Matrix::from([[6, 5, 4], [3, 2, 1]]);
            let trash = Matrix::from([[6, 5, 4, 3, 2, 1]]);

            assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
            assert_eq!(lhs, [[7; 3], [7; 3]]);
            assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
            assert_eq!(lhs, [[13, 12, 11], [10, 9, 8]]);
            assert_eq!(
                lhs.safe_add_assign(&trash),
                Err(crate::error::MatrixOperationError::NotSameSize(
                    Dimensions {
                        height: 2,
                        width: 3
                    },
                    Dimensions {
                        width: 6,
                        height: 1
                    }
                ))
            );
        }
        {
            let mut mat1 = Matrix::from([[1, 2], [3, 4]]);
            let mut mat2 = Matrix::from([[1, 6], [-12, -3]]);
            let mat3 = Matrix::from([[1, -4], [-2, 4]]);
            assert_eq!(mat2.safe_add_assign(&mat3), Ok(()));
            assert_eq!(mat2, [[2, 2], [-14, 1]]);
            assert_eq!(mat1.safe_add_assign(&mat2), Ok(()));
            assert_eq!(mat1, [[3, 4], [-11, 5]]);
        }
    }

    #[test]
    fn safe_sub_assign() {
        {
            let mut lhs = Matrix::from([[1, 2, 3], [4, 5, 6]]);
            let rhs = Matrix::from([[6, 5, 4], [3, 2, 1]]);
            let trash = Matrix::from([[6, 5, 4, 3, 2, 1]]);

            assert_eq!(lhs.safe_sub_assign(&rhs), Ok(()));
            assert_eq!(lhs, [[-5, -3, -1], [1, 3, 5]]);
            assert_eq!(lhs.safe_sub_assign(&rhs), Ok(()));
            assert_eq!(lhs, [[-11, -8, -5], [-2, 1, 4]]);
            assert_eq!(
                lhs.safe_sub_assign(&trash),
                Err(crate::error::MatrixOperationError::NotSameSize(
                    Dimensions {
                        height: 2,
                        width: 3
                    },
                    Dimensions {
                        width: 6,
                        height: 1
                    }
                ))
            );
        }
        {
            let mut mat1 = Matrix::from([[1, 2], [3, 4]]);
            let mut mat2 = Matrix::from([[1, 6], [-12, -3]]);
            let mat3 = Matrix::from([[1, -4], [-2, 4]]);
            assert_eq!(mat2.safe_sub_assign(&mat3), Ok(()));
            assert_eq!(mat2, [[0, 10], [-10, -7]]);
            assert_eq!(mat1.safe_sub_assign(&mat2), Ok(()));
            assert_eq!(mat1, [[1, -8], [13, 11]]);
        }
    }

    #[test]
    fn mul_assign() {
        let mut mat1 = Matrix::from([[1, 5], [8, 4]]);
        mat1 *= 4;
        assert_eq!(mat1, [[4, 20], [32, 16]]);
    }

    #[test]
    fn traits() {
        let mat1 = Matrix::from([[10, 1, 5], [5, 3, 8]]);
        let mat2 = mat1.clone() * 5;
        assert_eq!(mat2, [[50, 5, 25], [25, 15, 40]]);
        let mat3 = mat2.clone() + &mat1;
        assert_eq!(mat3, [[60, 6, 30], [30, 18, 48]]);
        let mat4 = mat3 - &mat1;
        assert_eq!(mat4, mat2);
    }

    #[test]
    fn example() {
        {
            let mut u = Matrix::from([[1., 2.], [3., 4.]]);
            let v = Matrix::from([[7., 4.], [-2., 2.]]);
            u += v;
            println!("{u}");
            assert_eq!(u, [[8., 6.], [1., 6.]]);
        }
        {
            let mut u = Matrix::from([[1., 2.], [3., 4.]]);
            let v = Matrix::from([[7., 4.], [-2., 2.]]);
            u -= v;
            println!("{u}");
            assert_eq!(u, [[-6., -2.], [5., 2.]]);
        }
        {
            let mut u = Matrix::from([[1., 2.], [3., 4.]]);
            u *= 2.;
            println!("{u}");
            assert_eq!(u, [[2., 4.], [6., 8.]]);
        }
    }

    #[test]
    fn add_with_complex() {
        let mut u = Matrix::from([[cpl!(1., 2.), cpl!(3., 4.)], [cpl!(5., 6.), cpl!(7., 8.)]]);
        let v = Matrix::from([[cpl!(8., 7.), cpl!(6., 5.)], [cpl!(4., 3.), cpl!(2., 1.)]]);
        u += &v;
        assert_eq!(u, Matrix::fill(cpl!(9., 9.), 2, 2).unwrap());
    }

    #[test]
    fn sub_with_complex() {
        let mut u = Matrix::from([[cpl!(1., 2.), cpl!(3., 4.)], [cpl!(5., 6.), cpl!(7., 8.)]]);
        let v = Matrix::from([[cpl!(8., 7.), cpl!(6., 5.)], [cpl!(4., 3.), cpl!(2., 1.)]]);
        u -= &v;
        assert_eq!(
            u,
            [
                [cpl!(-7., -5.), cpl!(-3., -1.)],
                [cpl!(1., 3.), cpl!(5., 7.)]
            ]
        );
    }

    #[test]
    fn scale_mul_with_complex() {
        let mut u = Matrix::from([[cpl!(1., 2.), cpl!(3., 4.)], [cpl!(5., 6.), cpl!(7., 8.)]]);
        u *= cpl!(5., 2.);
        assert_eq!(
            u,
            [
                [cpl!(1., 12.), cpl!(7., 26.)],
                [cpl!(13., 40.), cpl!(19., 54.)]
            ]
        );
    }
}
