mod ex00;
mod utils;
use crate::traits::Space;

///
/// A structure that store the size of a Matrix
///
#[derive(Clone, Copy, PartialEq, Debug, Default, Hash)]
pub struct Dimensions {
    /// The number of `columns` of the `Matrix`
    pub width: usize,
    /// The number of `lines` of the `Matrix`
    pub height: usize,
}

#[derive(Debug, Clone, PartialEq)]
///
/// Implementation of a Matrix
///
pub struct Matrix<K: Space> {
    content: Vec<K>,
    dimensions: Dimensions,
}
