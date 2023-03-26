use crate::matrix::Dimensions;

///
/// Describes the reason for which a [crate::Vector] operation can fail.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VectorOperationError {
    /// Contains the `size` of `(lhs, rhs)`
    NotSameSize(usize, usize),
}

///
/// Describes the reason for which a [crate::Matrix] operation can fail.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatrixOperationError {
    /// Contains the `size` of `(lhs, rhs)`
    NotSameSize(Dimensions, Dimensions),
}

///
/// Describes the reason for which a linear combination can fail.
///
/// See [crate::vector::safe_linear_combination].
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinearCombinationError {
    /// Contains the `size` of `(vectors, coefficients)`
    VectorsAndCoefficientSizeDifference(usize, usize),
    VectorArrayIsEmpty,
    /// Contains the `size` of `(first_vector, first_mismatch)`
    VectorSizeMismatch(usize, usize),
}

///
/// Describes the reason a linear interpolation
///
/// See [crate::utils::safe_lerp].
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinearInterpolationError {
    RatioOffBound,
}
