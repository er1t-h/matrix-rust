use crate::{
    static_asserts::{AssertNonZero, AssertNonZeroSizeType},
    traits::BasicValue,
};

use super::ConstMatrix;

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> From<[[K; COL_NUMBER]; ROW_NUMBER]>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
{
    #[allow(clippy::no_effect, path_statements)]
    fn from(matrix: [[K; COL_NUMBER]; ROW_NUMBER]) -> Self {
        AssertNonZero::<COL_NUMBER>::OK;
        AssertNonZero::<ROW_NUMBER>::OK;
        AssertNonZeroSizeType::<K>::OK;

        Self { content: matrix }
    }
}

impl<K: BasicValue, const SIZE: usize> ConstMatrix<K, SIZE, SIZE> {
    #[must_use]
    pub fn identity() -> Self {
        let array = std::array::from_fn(|y| {
            std::array::from_fn(|x| if x == y { K::one() } else { K::zero() })
        });
        Self::from(array)
    }
}

#[cfg(test)]
mod test {
    use crate::const_matrix::ConstMatrix;

    #[test]
    fn identity() {
        let mat = ConstMatrix::<u64, 3, 3>::identity();
        assert_eq!(mat, ConstMatrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1],]));
    }
}
