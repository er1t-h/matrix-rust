use crate::static_asserts::AssertNonZero;

mod column;
mod operations;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConstMatrix<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> {
    content: [[K; COL_NUMBER]; ROW_NUMBER],
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> From<[[K; COL_NUMBER]; ROW_NUMBER]>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
{
    #[allow(clippy::no_effect, path_statements)]
    fn from(matrix: [[K; COL_NUMBER]; ROW_NUMBER]) -> Self {
        AssertNonZero::<COL_NUMBER>::OK;
        AssertNonZero::<ROW_NUMBER>::OK;

        Self { content: matrix }
    }
}

#[cfg(test)]
mod test {
    use super::ConstMatrix;

    #[test]
    fn no_size() {
        let mat: ConstMatrix<i32, 0, 0> = ConstMatrix::from([]);
    }
}
