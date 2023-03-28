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
