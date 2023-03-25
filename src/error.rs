use crate::matrix::Dimensions;

///
/// Contains all kinds of error an operation between two vector can return
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VectorOperationError {
    /// Contains the `size` of `(lhs, rhs)`
    NotSameSize(usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatrixOperationError {
    /// Contains the `size` of `(lhs, rhs)`
    NotSameSize(Dimensions, Dimensions),
}
