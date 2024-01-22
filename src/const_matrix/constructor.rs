use crate::{
    static_asserts::{AssertNonZero, AssertNonZeroSizeType, AssertOperationEqual},
    traits::BasicValue,
};

use super::ConstMatrix;

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> From<[[K; COL_NUMBER]; ROW_NUMBER]>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
{
    #[must_use]
    fn from(matrix: [[K; COL_NUMBER]; ROW_NUMBER]) -> Self {
        #[allow(clippy::no_effect, path_statements)]
        {
            AssertNonZero::<COL_NUMBER>::OK;
            AssertNonZero::<ROW_NUMBER>::OK;
            AssertNonZeroSizeType::<K>::OK;
        }

        Self { content: matrix }
    }
}

impl<K: BasicValue, const SIZE: usize> ConstMatrix<K, SIZE, SIZE> {
    ///
    /// Returns an identity matrix
    ///
    #[must_use]
    pub fn identity() -> Self {
        #[allow(clippy::no_effect, path_statements)]
        {
            AssertNonZero::<SIZE>::OK;
            AssertNonZeroSizeType::<K>::OK;
        }

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
    #[must_use]
    pub fn augmented<const COL_NUMBER_LHS: usize, const COL_NUMBER_RHS: usize>(
        lhs: ConstMatrix<K, ROW_NUMBER, COL_NUMBER_LHS>,
        rhs: ConstMatrix<K, ROW_NUMBER, COL_NUMBER_RHS>,
    ) -> Self {
        #[allow(clippy::no_effect, path_statements)]
        {
            AssertNonZero::<COL_NUMBER>::OK;
            AssertNonZero::<ROW_NUMBER>::OK;
            AssertNonZeroSizeType::<K>::OK;
            AssertOperationEqual::<COL_NUMBER_LHS, COL_NUMBER_RHS, COL_NUMBER>::ADD;
        }

        // `lhs_iter` and `rhs_iter` will yield exactly `ROW_NUMBER` items
        let mut lhs_iter = lhs.content.into_iter();
        let mut rhs_iter = rhs.content.into_iter();
        // std::array::from_fn will call the callback exactly `ROW_NUMBER` times
        let content = std::array::from_fn(|_| {
            let line_lhs = lhs_iter.next().unwrap_or_else(|| unreachable!());
            let line_rhs = rhs_iter.next().unwrap_or_else(|| unreachable!());
            // `iter` will yield `COL_NUMBER_LHS + COL_NUMBER_RHS` elements
            // We know that `COL_NUMBER_LHS + COL_NUMBER_RHS = COL_NUMBER`
            let mut iter = line_lhs.into_iter().chain(line_rhs);
            std::array::from_fn(|_| iter.next().unwrap_or_else(|| unreachable!()))
        });
        Self { content }
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
}