use crate::matrix::Dimensions;

///
/// Describes the reason for which a [Vector](crate::Vector) operation can fail.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VectorOperationError {
    /// Contains the `size` of `(lhs, rhs)`
    NotSameSize(usize, usize),
    /// The Vector only contains 0s
    ZeroVector,
}

///
/// Describes the reason for which a [Matrix](crate::Matrix) operation can fail.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatrixOperationError {
    /// Contains the `size` of `(lhs, rhs)`
    NotSameSize(Dimensions, Dimensions),
}

///
/// Describes the reason for which a linear combination can fail.
///
/// See [linear_combination](crate::vector::linear_combination).
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
/// Describes the reason a linear interpolation can fail.
///
/// See [lerp](crate::utils::lerp).
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinearInterpolationError {
    RatioOffBound,
}

///
/// Describes the reason a cross product can fail.
///
/// See [cross_product](crate::utils::cross_product).
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CrossProductError {
    LeftVectorShouldBeThreeDimensional,
    RightVectorShouldBeThreeDimensional,
}

///
/// Describes the reason the multiplication of a [Matrix](crate::Matrix) by a
/// [Vector](crate::Vector) can fail.
///
/// See [mul_vec](crate::Matrix#method.mul_vec).
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MulVecError {
    /// Contains the `(number_of_matrix_column, vector_size)`
    SizeMismatch(usize, usize),
}

///
/// Describes the reason the multiplication of a [Matrix](crate::Matrix) by
/// another one can fail.
///
/// See [mul_mat](crate::Matrix#method.mul_mat).
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MulMatError {
    /// Contains the `(left_matrix_column, right_matrix_line)`
    SizeMismatch(usize, usize),
}

///
/// Describes the reason the trace of a [Matrix](crate::Matrix) can fail.
///
/// See [trace](crate::Matrix#method.trace).
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TraceError {
    NotSquareMatrix,
}

///
/// Describes the reason the determinant of a [Matrix](crate::Matrix) can fail.
///
/// See [`determinant`](crate::Matrix#method.determinant).
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeterminantError {
    NotSquareMatrix,
}

///
/// Describes the reason the inverse of a [Matrix](crate::Matrix) can fail.
///
/// See [`inverse`](crate::Matrix#method.inverse).
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InverseError {
    NotSquareMatrix,
    SingularMatrix,
}
