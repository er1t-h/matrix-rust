use crate::{
    angle::{Degree, Radian},
    const_vector::Vec3,
    static_asserts::{AssertCompare, AssertNonZero, AssertNonZeroSizeType, AssertOperationEqual},
    traits::BasicValue,
};

use super::ConstMatrix;

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

        let mut lhs_iter = lhs.content.into_iter();
        let mut rhs_iter = rhs.content.into_iter();
        let content = std::array::from_fn(|_| {
            let line_lhs = lhs_iter.next().unwrap();
            let line_rhs = rhs_iter.next().unwrap();
            let mut iter = line_lhs.into_iter().chain(line_rhs);
            std::array::from_fn(|_| iter.next().unwrap())
        });
        Self { content }
    }
}

impl ConstMatrix<f32, 3, 3> {
    pub fn from_axis_angle<A: Into<Radian>>([x, y, z]: [f32; 3], angle: A) -> Self {
        let (sin, cos) = angle.into().sin_cos();
        let osc = 1.0 - cos;
        ConstMatrix::from([
            [
                cos + x.powi(2) * osc,
                x * y * osc - z * sin,
                x * z * osc - y * sin,
            ],
            [
                y * x * osc + z * sin,
                cos + y.powi(2) * osc,
                y * z * osc - x * sin,
            ],
            [
                z * x * osc - y * sin,
                z * y * osc + x * sin,
                cos + z.powi(2) * osc,
            ],
        ])
    }
}

impl ConstMatrix<f32, 4, 4> {
    pub fn from_axis_angle<A: Into<Radian>>(axis: [f32; 3], angle: A) -> Self {
        ConstMatrix::<f32, 3, 3>::from_axis_angle(axis, angle).extend_identity(|| 0.0, || 1.0)
    }
}

impl ConstMatrix<f32, 4, 4> {
    pub fn projection<A: Into<Degree>>(fov: A, ratio: f32, near: f32, far: f32) -> Self {
        let fov = fov.into().0;
        Self::from([
            [1.0 / (ratio * (fov / 2.0).tan()), 0.0, 0.0, 0.0],
            [0.0, 1.0 / (fov / 2.0).tan(), 0.0, 0.0],
            [
                0.0,
                0.0,
                -((far + near) / (far - near)),
                -((2.0 * far * near) / (far - near)),
            ],
            [0.0, 0.0, -1.0, 0.0],
        ])
    }
}

impl ConstMatrix<f32, 4, 4> {
    pub fn look_at_rh(eye: Vec3, target: Vec3, up_vector: Vec3) -> Self {
        let forward_vector = (eye - target).normalize();
        let tmp = Vec3::from([0., 0., 1.]);
        let right_vector = forward_vector.cross(tmp).normalize();
        let up_vector = right_vector.cross(forward_vector);
        let translation_x = eye.dot(right_vector);
        let translation_y = eye.dot(up_vector);
        let translation_z = eye.dot(forward_vector);
        ConstMatrix::from([
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
    pub fn view_matrix(eye: Vec3, target: Vec3, up_vector: Vec3) -> Self {
        let forward_vector = (eye - target).normalize();
        let right_vector = up_vector.cross(forward_vector).normalize();
        // In Vulkan, the Y axis is reversed
        let up_vector = -right_vector.cross(forward_vector).normalize();
        let translation_x = eye.dot(right_vector);
        let translation_y = eye.dot(up_vector);
        let translation_z = eye.dot(forward_vector);
        ConstMatrix::from([
            [*right_vector.x(), *up_vector.x(), *forward_vector.x(), 0.],
            [*right_vector.y(), *up_vector.y(), *forward_vector.y(), 0.],
            [*right_vector.z(), *up_vector.z(), *forward_vector.z(), 0.],
            [-translation_x, -translation_y, -translation_z, 1.],
        ])
    }
}

impl<K, const SIZE_SRC: usize> ConstMatrix<K, SIZE_SRC, SIZE_SRC> {
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
        #[allow(path_statements)]
        {
            AssertCompare::<SIZE_SRC, SIZE_DEST>::LESS_THAN;
        }

        let mut iter_on_line = self.content.into_iter();
        ConstMatrix::from(std::array::from_fn(|line_index| {
            if let Some(line) = iter_on_line.next() {
                let mut iter_on_element =
                    line.into_iter().chain(std::iter::from_fn(|| Some(zero())));
                std::array::from_fn(|_| {
                    // into_iter will yield all elements, and iter::from_fn will yield indefinitely
                    iter_on_element.next().unwrap_or_else(|| unreachable!())
                })
            } else {
                let mut iter_on_element = std::iter::from_fn(|| Some(zero()))
                    .take(line_index)
                    .chain(std::iter::once_with(|| one()))
                    .chain(std::iter::from_fn(|| Some(zero())));
                std::array::from_fn(|_| iter_on_element.next().unwrap_or_else(|| unreachable!()))
            }
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
        assert_eq!(new_mat, expected)
    }
}
