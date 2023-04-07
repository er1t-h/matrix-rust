mod ex00;
mod ex07;
mod ex08;
mod ex09;
mod ex10;
mod ex11;
mod ex12;
mod ex13;
mod multiplicative_trace;
mod utils;

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
pub struct Matrix<K: Clone> {
    content: Vec<K>,
    dimensions: Dimensions,
}
