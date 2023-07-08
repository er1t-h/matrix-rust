mod operations;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConstMatrix<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> {
    content: [[K; COL_NUMBER]; ROW_NUMBER],
}

impl<K, const ROW_NUMBER: usize, const COL_NUMBER: usize> From<[[K; COL_NUMBER]; ROW_NUMBER]>
    for ConstMatrix<K, ROW_NUMBER, COL_NUMBER>
{
    fn from(matrix: [[K; COL_NUMBER]; ROW_NUMBER]) -> Self {
        Self { content: matrix }
    }
}
