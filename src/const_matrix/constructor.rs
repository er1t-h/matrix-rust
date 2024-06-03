use crate::{
    angle::Radian,
    const_vector::Vec3,
    static_asserts::{AssertCompare, AssertNonZero, AssertNonZeroSizeType, AssertOperationEqual},
    traits::BasicValue,
};

use super::{ConstMatrix, SquareMat};

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> From<[[K; COL_NUMBER]; ROW_NUMBER]>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
{
    #[allow(clippy::no_effect, path_statements)]
    #[must_use]
    fn from(matrix: [[K; COL_NUMBER]; ROW_NUMBER]) -> Self {
        AssertNonZero::<COL_NUMBER>::OK;
        AssertNonZero::<ROW_NUMBER>::OK;
        AssertNonZeroSizeType::<K>::OK;

        Self { content: matrix }
    }
}

impl<K: BasicValue, const SIZE: usize> ConstMatrix<K, SIZE, SIZE> {
    ///
    /// Returns an identity matrix
    ///
    #[must_use]
    pub fn identity() -> Self {
        let array = std::array::from_fn(|y| {
            std::array::from_fn(|x| if x == y { K::one() } else { K::zero() })
        });
        Self::from(array)
    }
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> ConstMatrix<K, ROW_NUMBER, COL_NUMBER> {
    ///
    /// Creates an augmented matrix from two other matrix
    ///
    /// # Panics
    /// Never.
    ///
    #[must_use]
    pub fn augmented<const COL_NUMBER_LHS: usize, const COL_NUMBER_RHS: usize>(
        lhs: ConstMatrix<K, ROW_NUMBER, COL_NUMBER_LHS>,
        rhs: ConstMatrix<K, ROW_NUMBER, COL_NUMBER_RHS>,
    ) -> Self {
        #[allow(clippy::no_effect, path_statements)]
        {
            AssertOperationEqual::<COL_NUMBER_LHS, COL_NUMBER_RHS, COL_NUMBER>::ADD;
        }

        // We know that lhs_iter and rhs_iter will yield ROW_NUMBER items
        let mut lhs_iter = lhs.content.into_iter();
        let mut rhs_iter = rhs.content.into_iter();
        // This will loop ROW_NUMBER times
        let content = std::array::from_fn(|_| {
            // So this unwrap is safe
            let line_lhs = lhs_iter.next().unwrap_or_else(|| unreachable!());
            let line_rhs = rhs_iter.next().unwrap_or_else(|| unreachable!());
            let mut iter = line_lhs.into_iter().chain(line_rhs);
            // We know that iter will yield COL_NUMBER_LHS + COL_NUMBER_RHS (thanks to the constant time assertion)
            std::array::from_fn(|_| iter.next().unwrap_or_else(|| unreachable!()))
        });
        Self { content }
    }
}

impl<T: Default> SquareMat<T, 2> {
    #[must_use]
    pub fn scale(x: T, y: T) -> Self {
        Self::generic_scale(x, y, Default::default)
    }
}
impl<T> SquareMat<T, 2> {
    #[must_use]
    pub fn generic_scale(x: T, y: T, zero: impl Fn() -> T) -> Self {
        Self::from([[x, zero()], [zero(), y]])
    }
}

impl SquareMat<f32, 3> {
    pub fn from_axis_angle<A: Into<Radian>>([x, y, z]: [f32; 3], angle: A) -> Self {
        let (sin, cos) = angle.into().sin_cos();
        let osc = 1.0 - cos;
        Self::from([
            [
                x.powi(2).mul_add(osc, cos),
                (x * y).mul_add(osc, -(z * sin)),
                (x * z).mul_add(osc, y * sin),
            ],
            [
                (y * x).mul_add(osc, z * sin),
                y.powi(2).mul_add(osc, cos),
                (y * z).mul_add(osc, -(x * sin)),
            ],
            [
                (z * x).mul_add(osc, -(y * sin)),
                (z * y).mul_add(osc, x * sin),
                z.powi(2).mul_add(osc, cos),
            ],
        ])
    }

    pub fn rotation(
        rotation_x: impl Into<Radian>,
        rotation_y: impl Into<Radian>,
        rotation_z: impl Into<Radian>,
    ) -> Self {
        let (sin_x, cos_x) = rotation_x.into().sin_cos();
        let (sin_y, cos_y) = rotation_y.into().sin_cos();
        let (sin_z, cos_z) = rotation_z.into().sin_cos();
        Self::from([
            [
                cos_z * cos_y,
                (cos_z * sin_y).mul_add(sin_x, -(sin_z * cos_x)),
                (cos_z * sin_y).mul_add(cos_x, sin_z * sin_x),
            ],
            [
                sin_z * cos_y,
                (sin_z * sin_y).mul_add(sin_x, cos_z * cos_x),
                (sin_z * sin_y).mul_add(cos_x, -(cos_z * sin_x)),
            ],
            [-sin_y, cos_y * sin_x, cos_y * cos_x],
        ])
    }

    pub fn rotation_x(rotation: impl Into<Radian>) -> Self {
        let (sin, cos) = rotation.into().sin_cos();

        Self::from([[1., 0., 0.], [0., cos, -sin], [0., sin, cos]])
    }

    pub fn rotation_y(rotation: impl Into<Radian>) -> Self {
        let (sin, cos) = rotation.into().sin_cos();

        Self::from([[cos, 0., sin], [0., 1., 0.], [-sin, 0., cos]])
    }

    pub fn rotation_z(rotation: impl Into<Radian>) -> Self {
        let (sin, cos) = rotation.into().sin_cos();

        Self::from([[cos, -sin, 0.], [sin, cos, 0.], [0., 0., 1.]])
    }

    #[must_use]
    pub fn translation(x: f32, y: f32) -> Self {
        Self::from([[1., 0., x], [0., 1., y], [0., 0., 1.]])
    }
}

impl<T> SquareMat<T, 3> {
    #[must_use]
    pub fn generic_translation(x: T, y: T, zero: impl Fn() -> T, one: impl Fn() -> T) -> Self {
        Self::from([
            [one(), zero(), x],
            [zero(), one(), y],
            [zero(), zero(), one()],
        ])
    }

    #[must_use]
    pub fn generic_scale(x: T, y: T, z: T, zero: impl Fn() -> T) -> Self {
        Self::from([
            [x, zero(), zero()],
            [zero(), y, zero()],
            [zero(), zero(), z],
        ])
    }
}

impl<T: Default> SquareMat<T, 3> {
    #[must_use]
    pub fn scale(x: T, y: T, z: T) -> Self {
        Self::generic_scale(x, y, z, Default::default)
    }
}

impl<T> SquareMat<T, 4> {
    #[must_use]
    pub fn generic_translation(
        x: T,
        y: T,
        z: T,
        zero: impl Fn() -> T,
        one: impl Fn() -> T,
    ) -> Self {
        Self::from([
            [one(), zero(), zero(), x],
            [zero(), one(), zero(), y],
            [zero(), zero(), one(), z],
            [zero(), zero(), zero(), one()],
        ])
    }
}

impl SquareMat<f32, 4> {
    pub fn from_axis_angle<A: Into<Radian>>(axis: [f32; 3], angle: A) -> Self {
        SquareMat::<f32, 3>::from_axis_angle(axis, angle).extend_identity(|| 0.0, || 1.0)
    }

    pub fn rotation(
        rotation_x: impl Into<Radian>,
        rotation_y: impl Into<Radian>,
        rotation_z: impl Into<Radian>,
    ) -> Self {
        SquareMat::<f32, 3>::rotation(rotation_x, rotation_y, rotation_z)
            .extend_identity(|| 0.0, || 1.0)
    }

    pub fn rotation_x(rotation: impl Into<Radian>) -> Self {
        SquareMat::<f32, 3>::rotation_x(rotation).extend_identity(|| 0.0, || 1.0)
    }

    pub fn rotation_y(rotation: impl Into<Radian>) -> Self {
        SquareMat::<f32, 3>::rotation_y(rotation).extend_identity(|| 0.0, || 1.0)
    }

    pub fn rotation_z(rotation: impl Into<Radian>) -> Self {
        SquareMat::<f32, 3>::rotation_z(rotation).extend_identity(|| 0.0, || 1.0)
    }

    pub fn projection<A: Into<Radian>>(fov: A, ratio: f32, near: f32, far: f32) -> Self {
        let fov = fov.into().0;
        Self::from([
            [1.0 / (ratio * (fov / 2.0).tan()), 0.0, 0.0, 0.0],
            [0.0, -(1.0 / (fov / 2.0).tan()), 0.0, 0.0],
            [
                0.0,
                0.0,
                -((far + near) / (far - near)),
                -((2.0 * far * near) / (far - near)),
            ],
            [0.0, 0.0, -1.0, 0.0],
        ])
    }
    #[must_use]
    pub fn look_at_rh(eye: Vec3, target: Vec3, _up_vector: Vec3) -> Self {
        let forward_vector = (eye - target).normalize();
        let tmp = Vec3::from([0., 0., 1.]);
        let right_vector = forward_vector.cross(&tmp).normalize();
        let up_vector = right_vector.cross(&forward_vector);
        let translation_x = eye.dot(right_vector);
        let translation_y = eye.dot(up_vector);
        let translation_z = eye.dot(forward_vector);
        Self::from([
            [
                *right_vector.x(),
                *right_vector.y(),
                *right_vector.z(),
                translation_x,
            ],
            [
                *up_vector.x(),
                *up_vector.y(),
                *up_vector.z(),
                translation_y,
            ],
            [
                *forward_vector.x(),
                *forward_vector.y(),
                *forward_vector.z(),
                translation_z,
            ],
            [0., 0., 0., 1.],
        ])
    }

    ///
    /// Returns a Vulkan view matrix.
    ///
    #[must_use]
    pub fn view_matrix(eye: Vec3, target: Vec3, up_vector: Vec3) -> Self {
        let forward_vector = (eye - target).normalize();
        let right_vector = up_vector.cross(&forward_vector).normalize();
        // In Vulkan, the Y axis is reversed
        let up_vector = -right_vector.cross(&forward_vector).normalize();
        let translation_x = eye.dot(right_vector);
        let translation_y = eye.dot(up_vector);
        let translation_z = eye.dot(forward_vector);
        Self::from([
            [*right_vector.x(), *up_vector.x(), *forward_vector.x(), 0.],
            [*right_vector.y(), *up_vector.y(), *forward_vector.y(), 0.],
            [*right_vector.z(), *up_vector.z(), *forward_vector.z(), 0.],
            [-translation_x, -translation_y, -translation_z, 1.],
        ])
    }

    #[must_use]
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self::from([
            [1., 0., 0., x],
            [0., 1., 0., y],
            [0., 0., 1., z],
            [0., 0., 0., 1.],
        ])
    }
}

impl<K, const SIZE_SRC: usize> SquareMat<K, SIZE_SRC> {
    ///
    /// Extends the square matrix by adding ones on the diagonal, and zero otherwise
    ///
    /// # Example
    /// ```
    /// use matrix::ConstMatrix;
    /// ```
    ///
    pub fn extend_identity<const SIZE_DEST: usize, Zero: Fn() -> K, One: Fn() -> K>(
        self,
        zero: Zero,
        one: One,
    ) -> ConstMatrix<K, SIZE_DEST, SIZE_DEST> {
        #[allow(path_statements, clippy::no_effect)]
        {
            AssertCompare::<SIZE_SRC, SIZE_DEST>::LESS_THAN;
        }

        let mut iter_on_line = self.content.into_iter();
        SquareMat::from(std::array::from_fn(|line_index| {
            iter_on_line.next().map_or_else(
                || {
                    let mut iter_on_element = std::iter::from_fn(|| Some(zero()))
                        .take(line_index)
                        .chain(std::iter::once_with(&one))
                        .chain(std::iter::from_fn(|| Some(zero())));
                    std::array::from_fn(|_| {
                        iter_on_element.next().unwrap_or_else(|| unreachable!())
                    })
                },
                |line| {
                    let mut iter_on_element =
                        line.into_iter().chain(std::iter::from_fn(|| Some(zero())));
                    std::array::from_fn(|_| {
                        // into_iter will yield all elements, and iter::from_fn will yield indefinitely
                        iter_on_element.next().unwrap_or_else(|| unreachable!())
                    })
                },
            )
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::const_matrix::ConstMatrix;

    #[test]
    fn identity() {
        let mat = ConstMatrix::<u64, 2, 2>::identity();
        assert_eq!(mat, ConstMatrix::from([[1, 0], [0, 1]]));
        let mat = ConstMatrix::<u64, 3, 3>::identity();
        assert_eq!(mat, ConstMatrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1],]));
        let mat = ConstMatrix::<u64, 4, 4>::identity();
        assert_eq!(
            mat,
            ConstMatrix::from([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]])
        );
    }

    #[test]
    fn augmented() {
        let lhs = ConstMatrix::from([[1], [2]]);
        let rhs = ConstMatrix::<u64, 2, 2>::identity();
        let result: ConstMatrix<u64, 2, 3> = ConstMatrix::augmented(lhs, rhs);
        assert_eq!(result, ConstMatrix::from([[1, 1, 0], [2, 0, 1]]));
    }

    #[test]
    fn extend_identity() {
        let mat = ConstMatrix::from([[1, 2], [3, 4]]);
        let new_mat = mat.extend_identity(|| 0, || 1);
        let expected = ConstMatrix::from([[1, 2, 0, 0], [3, 4, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]);
        assert_eq!(new_mat, expected);
    }
}
