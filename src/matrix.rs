mod basic_operations;
mod determinant;
mod inverse;
mod matrix_mul;
mod multiplicative_trace;
mod rank;
mod reduced_row_echelon;
mod trace;
mod transpose;
mod utils;
mod vector_mul;

pub use utils::TermByTermMul;

///
/// A structure that store the size of a Matrix
///
#[derive(Clone, Copy, PartialEq, Debug, Default, Hash, Eq)]
pub struct Dimensions {
    /// The number of `columns` of the `Matrix`
    pub width: usize,
    /// The number of `lines` of the `Matrix`
    pub height: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
///
/// Implementation of a Matrix
///
#[must_use]
pub struct Matrix<K: Clone> {
    content: Vec<K>,
    dimensions: Dimensions,
}

impl<K: Clone> Matrix<K> {
    #[must_use]
    pub const fn dimensions(&self) -> Dimensions {
        self.dimensions
    }
}
