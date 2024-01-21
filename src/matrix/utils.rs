use std::{
    error::Error,
    fmt::Display,
    ops::{Add, Deref, DerefMut, MulAssign, Range, Sub},
    slice::{Iter, IterMut},
};

use crate::{
    error::{AugmentedMatrixError, SubmatrixError, WithoutLineColumnError},
    Matrix, Vector,
};

use self::{
    columns::{MatrixColumn, MatrixColumnMut},
    iterator::{MatrixIterByColumn, MatrixIterByColumnMut},
};
use super::Dimensions;

pub mod columns;
pub mod iterator;

pub trait TermByTermMul<Rhs = Self>: Sized {
    type Error;
    ///
    /// Multiplies term by term two objects
    ///
    /// # Errors
    /// If an error is to occur, it should be returned as a `Self::Error`
    ///
    fn mul_assign_term_by_term(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
    ///
    /// Returns the term by term multiplication of two objects
    ///
    /// # Errors
    /// If an error is to occur, it should be returned as a `Self::Error`
    ///
    fn mul_term_by_term(mut self, rhs: Rhs) -> Result<Self, Self::Error> {
        self.mul_assign_term_by_term(rhs).map(|_| self)
    }
}

impl<K> TermByTermMul for Matrix<K>
where
    K: Clone + MulAssign<K>,
{
    type Error = ();
    fn mul_assign_term_by_term(&mut self, rhs: Self) -> Result<(), Self::Error> {
        if self.dimensions() != rhs.dimensions() {
            return Err(());
        }
        for (lhs, rhs) in self.iter_mut().zip(rhs) {
            *lhs *= rhs;
        }
        Ok(())
    }
}

impl<'a, K> TermByTermMul<&'a Self> for Matrix<K>
where
    K: Clone + MulAssign<&'a K>,
{
    type Error = ();
    fn mul_assign_term_by_term(&mut self, rhs: &'a Self) -> Result<(), Self::Error> {
        if self.dimensions() != rhs.dimensions() {
            return Err(());
        }
        for (lhs, rhs) in self.iter_mut().zip(rhs) {
            *lhs *= rhs;
        }
        Ok(())
    }
}

// Display
impl<K: Clone + Display> Display for Matrix<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.content.is_empty() {
            return write!(f, "[[]]");
        }
        let mut str = String::new();
        for y in (0..self.content.len()).step_by(self.dimensions.width) {
            str.push('[');
            str += &self.content[y..y + self.dimensions.width]
                .iter()
                .map(ToString::to_string)
                .reduce(|accumulator, elt| accumulator + ", " + &elt)
                .unwrap();
            str += "]; ";
        }
        write!(f, "[{}]", &str[..str.len() - 2])
    }
}

// From [[K]]
impl<K: Clone, const LINE_SIZE: usize, const COLUMN_SIZE: usize> From<[[K; COLUMN_SIZE]; LINE_SIZE]>
    for Matrix<K>
{
    #[inline(always)]
    fn from(value: [[K; COLUMN_SIZE]; LINE_SIZE]) -> Self {
        Self {
            content: value.into_iter().flatten().collect(),
            dimensions: Dimensions {
                width: COLUMN_SIZE,
                height: LINE_SIZE,
            },
        }
    }
}
impl<K: Clone, const LINE_SIZE: usize> From<&[[K; LINE_SIZE]]> for Matrix<K> {
    #[inline(always)]
    fn from(value: &[[K; LINE_SIZE]]) -> Self {
        Self {
            content: value.iter().flatten().cloned().collect(),
            dimensions: Dimensions {
                width: value.len(),
                height: LINE_SIZE,
            },
        }
    }
}
impl<K: Clone> From<&[&[K]]> for Matrix<K> {
    #[inline(always)]
    fn from(value: &[&[K]]) -> Self {
        Self {
            content: value.iter().flat_map(|x| *x).cloned().collect(),
            dimensions: Dimensions {
                width: value.len(),
                height: value.get(0).map_or(0, |x| x.len()),
            },
        }
    }
}

impl<K: Clone> From<Vector<K>> for Matrix<K> {
    #[inline(always)]
    fn from(value: Vector<K>) -> Self {
        let len = value.len();

        Self {
            content: value.into_iter().collect(),
            dimensions: Dimensions {
                width: 1,
                height: len,
            },
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum MatrixCreationError {
    EmptyMatrix,
    NotEqualLines,
}
impl Error for MatrixCreationError {}
impl Display for MatrixCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyMatrix => write!(f, "empty matrix."),
            Self::NotEqualLines => write!(f, "not all lines have the same size."),
        }
    }
}
impl<K: Clone> TryFrom<Vec<Vec<K>>> for Matrix<K> {
    type Error = MatrixCreationError;
    #[inline(always)]
    fn try_from(value: Vec<Vec<K>>) -> Result<Self, Self::Error> {
        let height = value.len();
        if height == 0 {
            return Err(MatrixCreationError::EmptyMatrix);
        }
        let width = value[0].len();
        if width == 0 {
            return Err(MatrixCreationError::EmptyMatrix);
        }
        if value.iter().skip(1).any(|x| x.len() != width) {
            return Err(MatrixCreationError::NotEqualLines);
        }
        Ok(Self {
            content: value.into_iter().flatten().collect(),
            dimensions: Dimensions { width, height },
        })
    }
}

//IntoIters
impl<K: Clone> IntoIterator for Matrix<K> {
    type Item = K;
    type IntoIter = <Vec<K> as IntoIterator>::IntoIter;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}
impl<'a, K: Clone> IntoIterator for &'a Matrix<K> {
    type Item = &'a K;
    type IntoIter = <&'a Vec<K> as IntoIterator>::IntoIter;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.content.iter()
    }
}
impl<'a, K: Clone> IntoIterator for &'a mut Matrix<K> {
    type Item = &'a mut K;
    type IntoIter = <&'a mut Vec<K> as IntoIterator>::IntoIter;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.content.iter_mut()
    }
}

// Derefs
impl<K: Clone> Deref for Matrix<K> {
    type Target = [K];
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<K: Clone> DerefMut for Matrix<K> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

// impl<K> PartialEq<[[K; COLUMN_SIZE]; LINE_SIZE]>
//     for Matrix<K>
// where
//     K: Clone + PartialEq<K>,
// {
//     fn eq(&self, other: &[[K; COLUMN_SIZE]]) -> bool {
//         self.dimensions.width == COLUMN_SIZE && self.dimensions.height == LINE_SIZE &&
//         Iterator::eq(self.iter(), other.iter().flatten())
//     }
// }
// impl<K, const COLUMN_SIZE: usize> PartialEq<[[K; COLUMN_SIZE]; LINE_SIZE]>
//     for Matrix<K>
// where
//     K: Clone + PartialEq<K>,
// {
//     fn eq(&self, other: &[[K; COLUMN_SIZE]]) -> bool {
//         self.dimensions.width == COLUMN_SIZE && self.dimensions.height == LINE_SIZE &&
//         Iterator::eq(self.iter(), other.iter().flatten())
//     }
// }
impl<K, const LINE_SIZE: usize, const COLUMN_SIZE: usize> PartialEq<[[K; COLUMN_SIZE]; LINE_SIZE]>
    for Matrix<K>
where
    K: Clone + PartialEq<K>,
{
    fn eq(&self, other: &[[K; COLUMN_SIZE]; LINE_SIZE]) -> bool {
        self.dimensions.width == COLUMN_SIZE
            && self.dimensions.height == LINE_SIZE
            && Iterator::eq(self.iter(), other.iter().flatten())
    }
}

impl<K> Matrix<K>
where
    for<'a> K: Clone + PartialOrd<K>,
    for<'a> &'a K: Sub<&'a K, Output = K> + Add<&'a K, Output = K>,
{
    ///
    /// Returns true if each element of `other` are equal to those of `self`,
    /// plus or minus `delta`.
    ///
    pub fn approx_eq<const LINE_SIZE: usize, const COLUMN_SIZE: usize>(
        &self,
        other: &[[K; COLUMN_SIZE]; LINE_SIZE],
        delta: &K,
    ) -> bool {
        if !(self.dimensions.width == COLUMN_SIZE && self.dimensions.height == LINE_SIZE) {
            return false;
        }
        for (mat, cmp) in self.iter().zip(other.iter().flatten()) {
            let low = cmp - delta;
            let high = cmp + delta;
            if mat < &low || mat > &high {
                return false;
            }
        }
        true
    }
}

// Creating new matrices
impl<K> Matrix<K>
where
    K: Clone + Default,
{
    ///
    /// Creates an identity matrix, setting all elements to Default, and the
    /// diagonal to `diagonal_value`.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mat = Matrix::identity(&1., 3).unwrap();
    /// assert_eq!(mat, [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    /// let mat = Matrix::identity(&5., 2).unwrap();
    /// assert_eq!(mat, [[5., 0.], [0., 5.]]);
    /// ```
    ///
    pub fn identity(diagonal_value: &K, size: usize) -> Option<Self> {
        if size == 0 {
            return None;
        }
        let mut content: Vec<K> = Vec::with_capacity(size * size);
        for i in 0..size {
            for j in 0..size {
                if i == j {
                    content.push(diagonal_value.clone());
                } else {
                    content.push(K::default());
                }
            }
        }
        Some(Self {
            content,
            dimensions: Dimensions {
                width: size,
                height: size,
            },
        })
    }

    ///
    /// Creates a matrix filled with the default value of the type `K`.
    ///
    /// If one of the parameters is equal to 0, returns `None`.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mat: Matrix<f64> = Matrix::fill_default(3, 2).unwrap();
    /// assert_eq!(mat, [[0., 0., 0.], [0., 0., 0.]]);
    /// let mat = Matrix::fill_default(1, 2).unwrap();
    /// assert_eq!(mat, [[0.], [0.]]);
    /// ```
    ///
    #[must_use]
    pub fn fill_default(number_of_column: usize, number_of_line: usize) -> Option<Self> {
        if number_of_column == 0 || number_of_line == 0 {
            return None;
        }
        let total_size = number_of_column * number_of_line;
        let mut content: Vec<K> = Vec::with_capacity(total_size);
        content.resize(total_size, K::default());
        Some(Self {
            content,
            dimensions: Dimensions {
                width: number_of_column,
                height: number_of_line,
            },
        })
    }
}

// Utils
impl<'a, K: Clone> Matrix<K> {
    ///
    /// Returns the augmented matrix composed of `left` and `right`.
    ///
    /// # Panics
    /// Never.
    ///
    /// # Errors
    /// If the height of the two matrix doesn't match, returns a [`DimensionMismatch`](AugmentedMatrixError::DimensionMismatch)
    ///
    pub fn augmented_matrix(left: &Self, right: &Self) -> Result<Self, AugmentedMatrixError> {
        if left.dimensions.height != right.dimensions.height {
            return Err(AugmentedMatrixError::DimensionMismatch);
        }
        let mut content: Vec<K> = Vec::with_capacity(
            (left.dimensions.width + right.dimensions.width) * left.dimensions.height,
        );
        for i in 0..left.dimensions.height {
            content.extend(left.get_line(i).unwrap().cloned());
            content.extend(right.get_line(i).unwrap().cloned());
        }
        Ok(Self {
            content,
            dimensions: Dimensions {
                width: left.dimensions.width + right.dimensions.width,
                height: left.dimensions.height,
            },
        })
    }

    ///
    /// Returns the submatrix contained between `columns` and `lines`.
    ///
    /// # Panics
    /// Never.
    ///
    /// # Errors
    /// If one of the range is out of bounds, returns [`InvalidRanges`](SubmatrixError::InvalidRanges)
    ///
    pub fn submatrix(
        &self,
        columns: Range<usize>,
        lines: Range<usize>,
    ) -> Result<Self, SubmatrixError> {
        if self.dimensions.height < lines.end
            || self.dimensions.width < columns.end
            || columns.start == columns.end
            || lines.start == lines.end
        {
            return Err(SubmatrixError::InvalidRanges);
        }
        let column = (columns.start, columns.end);
        let line = (lines.start, lines.end);
        let mut content: Vec<K> =
            Vec::with_capacity((columns.end - columns.start) * (lines.end - lines.start));
        for line in self.content.windows(self.dimensions.width) {
            content.extend_from_slice(&line[columns.clone()]);
        }
        Ok(Self {
            content,
            dimensions: Dimensions {
                width: column.1 - column.0,
                height: line.1 - line.0,
            },
        })
    }

    ///
    /// Returns a clone of the matrix, without all elements on the line `line`
    /// or on the column `column`.
    ///
    /// # Errors
    /// If the matrix has only one line or one column, returns [`TooSmallMatrix`](WithoutLineColumnError::TooSmallMatrix)
    ///
    pub fn without_line_column(
        &self,
        column: usize,
        line: usize,
    ) -> Result<Self, WithoutLineColumnError> {
        if self.dimensions.height < 2 || self.dimensions.width < 2 {
            return Err(WithoutLineColumnError::TooSmallMatrix);
        }
        let content: Vec<K> = self
            .content
            .iter()
            .enumerate()
            .filter(|(index, _)| {
                index % self.dimensions.width != column && index / self.dimensions.width != line
            })
            .map(|(_, elt)| elt.clone())
            .collect();
        Ok(Self {
            content,
            dimensions: Dimensions {
                width: self.dimensions.width - 1,
                height: self.dimensions.height - 1,
            },
        })
    }

    ///
    /// Returns the number of (`lines`, `columns`) in the `Matrix`.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mat = Matrix::from([[1, 2, 3, 4], [5, 6, 7, 8]]);
    /// assert_eq!(mat.size(), (2, 4));
    /// ```
    ///
    /// # Complexity:
    /// Constant
    ///
    #[must_use]
    pub const fn size(&self) -> (usize, usize) {
        (self.dimensions.height, self.dimensions.width)
    }

    ///
    /// Swap the two lines given in parameter
    ///
    /// Panics if input is wrong (line out of bound)
    ///
    pub(super) fn swap_line(&mut self, first_line: usize, second_line: usize) {
        if first_line == second_line {
            return;
        }
        let (min, max) = (first_line.min(second_line), first_line.max(second_line));
        let split_index = self.dimensions.width * (min + 1);
        let (left, right) = self.content.as_mut_slice().split_at_mut(split_index);
        let max_slice_begin = max * self.dimensions.width - split_index;
        left[split_index - self.dimensions.width..]
            .swap_with_slice(&mut right[max_slice_begin..max_slice_begin + self.dimensions.width]);
    }

    ///
    /// Returns `true` if the matrix is a square, `false` otherwise
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let square_matrix = Matrix::from([[1, 2], [3, 4]]);
    /// let not_square_matrix = Matrix::from([[1, 2], [3, 4], [5, 6]]);
    /// assert!(square_matrix.is_square());
    /// assert!(!not_square_matrix.is_square());
    /// ```
    ///
    /// # Complexity:
    /// Constant
    ///
    #[must_use]
    pub const fn is_square(&self) -> bool {
        self.dimensions.height == self.dimensions.width
    }

    ///
    /// Returns a reference to the element at line `line` and column `column`,
    /// or None if it's outside bounds.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mat = Matrix::from([[1, 2], [3, 4]]);
    /// assert_eq!(mat.get(0, 0), Some(&1));
    /// assert_eq!(mat.get(0, 1), Some(&2));
    /// assert_eq!(mat.get(1, 0), Some(&3));
    /// assert_eq!(mat.get(1, 1), Some(&4));
    /// ```
    ///
    /// # Complexity:
    /// Constant
    ///
    #[must_use]
    pub fn get(&'a self, line: usize, column: usize) -> Option<&'a K> {
        if column < self.dimensions.width {
            self.content.get(line * self.dimensions.width + column)
        } else {
            None
        }
    }
    ///
    /// Returns a mutable reference to the element at line `line` and column
    /// `column`, or None if it's outside bounds.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mut mat = Matrix::from([[1, 2], [3, 4]]);
    /// *mat.get_mut(0, 0).unwrap() = 4;
    /// assert_eq!(mat.get(0, 0), Some(&4));
    /// ```
    ///
    /// # Complexity:
    /// Constant
    ///
    #[must_use]
    pub fn get_mut(&'a mut self, line: usize, column: usize) -> Option<&'a mut K> {
        if column < self.dimensions.width {
            self.content.get_mut(line * self.dimensions.width + column)
        } else {
            None
        }
    }

    ///
    /// Returns an iterator that go through the column `column_number`.
    ///
    /// Returns None if `column_number` is off bounds
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mat = Matrix::from([[1, 2], [3, 4]]);
    /// let mut iter = mat.get_column(0).unwrap();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// # Complexity:
    /// Constant
    ///
    #[must_use]
    pub fn get_column(&'a self, column_number: usize) -> Option<MatrixColumn<'a, K>> {
        if self.dimensions.width < column_number || self.dimensions.height == 0 {
            None
        } else {
            Some(MatrixColumn::new(
                &self.content,
                column_number,
                self.dimensions.width,
            ))
        }
    }

    ///
    /// Returns an iterator that go through the line `line_number`.
    ///
    /// Returns None if `line_number` is off bounds.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mat = Matrix::from([[1, 2], [3, 4]]);
    /// let mut iter = mat.get_line(1).unwrap();
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// # Complexity:
    /// Constant
    ///
    #[must_use]
    pub fn get_line(&'a self, line_number: usize) -> Option<Iter<'a, K>> {
        if self.dimensions.height < line_number || self.dimensions.width == 0 {
            None
        } else {
            let begin_line = self.dimensions.width * line_number;
            Some(self.content[begin_line..begin_line + self.dimensions.width].iter())
        }
    }

    ///
    /// Returns a slice of the line `line_number`.
    ///
    /// Returns None if `line_number` is off bounds.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mat = Matrix::from([[1, 2], [3, 4]]);
    /// let mut slice = mat.get_line_slice(1).unwrap();
    /// assert_eq!(slice, &[3, 4]);
    /// ```
    ///
    /// # Complexity:
    /// Constant
    ///
    #[must_use]
    pub fn get_line_slice(&self, line_number: usize) -> Option<&[K]> {
        if self.dimensions.height < line_number || self.dimensions.width == 0 {
            None
        } else {
            let begin_line = self.dimensions.width * line_number;
            Some(&self.content[begin_line..begin_line + self.dimensions.width])
        }
    }

    ///
    /// Returns an iterator that go through the column `column_number`, yielding
    /// mutable references
    ///
    /// Returns None if `column_number` is off bounds
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mut mat = Matrix::from([[1, 2], [3, 4]]);
    /// let mut iter = mat.get_column_mut(0).unwrap();
    /// *iter.next().unwrap() = 5;
    /// *iter.next().unwrap() = 6;
    /// assert_eq!(mat, [[5, 2], [6, 4]]);
    /// ```
    ///
    /// # Complexity:
    /// Constant
    ///
    pub fn get_column_mut(&'a mut self, column_number: usize) -> Option<MatrixColumnMut<'a, K>> {
        if self.dimensions.width < column_number || self.dimensions.height == 0 {
            None
        } else {
            Some(MatrixColumnMut::new(
                &mut self.content,
                column_number,
                self.dimensions.width,
            ))
        }
    }

    ///
    /// Returns an iterator that go through the line `line_number`.
    ///
    /// Returns None if `line_number` is off bounds.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mut mat = Matrix::from([[1, 2], [3, 4]]);
    /// let mut iter = mat.get_line_mut(1).unwrap();
    /// for elt in iter {
    ///     *elt *= 2;
    /// }
    /// assert_eq!(mat, [[1, 2], [6, 8]]);
    /// ```
    ///
    /// # Complexity:
    /// Constant
    ///
    pub fn get_line_mut(&'a mut self, line_number: usize) -> Option<IterMut<'a, K>> {
        if self.dimensions.height < line_number || self.dimensions.width == 0 {
            None
        } else {
            let begin_line = self.dimensions.width * line_number;
            Some(self.content[begin_line..begin_line + self.dimensions.width].iter_mut())
        }
    }

    ///
    /// Returns a mutable slice of the line `line_number`.
    ///
    /// Returns None if `line_number` is off bounds.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mat = Matrix::from([[1, 2], [3, 4]]);
    /// let mut slice = mat.get_line_slice(1).unwrap();
    /// assert_eq!(slice, &[3, 4]);
    /// ```
    ///
    /// # Complexity:
    /// Constant
    ///
    pub fn get_line_mut_slice(&mut self, line_number: usize) -> Option<&mut [K]> {
        if self.dimensions.height < line_number || self.dimensions.width == 0 {
            None
        } else {
            let begin_line = self.dimensions.width * line_number;
            Some(&mut self.content[begin_line..begin_line + self.dimensions.width])
        }
    }

    ///
    /// Returns an iterator over the `columns` of a `Matrix`
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mut mat = Matrix::from([[1, 2], [3, 4]]);
    /// let mut iter = mat.columns();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn columns(&'a self) -> MatrixIterByColumn<'a, K> {
        MatrixIterByColumn::new(self)
    }

    ///
    /// Returns a mutable iterator over the `columns` of a `Matrix`
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    /// let mut mat = Matrix::from([[1, 2], [3, 4]]);
    /// let mut iter = mat.columns_mut();
    /// *iter.next().unwrap() = 1;
    /// *iter.next().unwrap() = 2;
    /// *iter.next().unwrap() = 3;
    /// *iter.next().unwrap() = 4;
    /// let mut iter = mat.columns();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    #[inline(always)]
    pub fn columns_mut(&'a mut self) -> MatrixIterByColumnMut<'a, K> {
        MatrixIterByColumnMut::new(self)
    }

    ///
    /// Adds a column at the end of a matrix
    ///
    /// Returns true if the operation succeeded, false otherwise.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    /// let mut mat = Matrix::from([[1, 2], [4, 5]]);
    /// mat.append_column(&[3, 6]);
    /// assert_eq!(mat, [[1, 2, 3], [4, 5, 6]]);
    /// ```
    ///
    pub fn append_column(&mut self, content: &[K]) -> bool {
        if self.dimensions.height != content.len() {
            return false;
        }
        for (index, elt) in content.iter().enumerate().rev() {
            self.content
                .insert((index + 1) * self.dimensions.width, elt.clone());
        }
        self.dimensions.width += 1;
        true
    }

    ///
    /// Adds a column at the end of a matrix
    ///
    /// Returns true if the operation succeeded, false otherwise.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    /// let mut mat = Matrix::from([[1, 2], [3, 4]]);
    /// mat.append_line(&[5, 6]);
    /// assert_eq!(mat, [[1, 2], [3, 4], [5, 6]]);
    /// ```
    ///
    pub fn append_line(&mut self, content: &[K]) -> bool {
        if self.dimensions.width != content.len() {
            return false;
        }
        self.content.extend_from_slice(content);
        self.dimensions.height += 1;
        true
    }

    ///
    /// Creates a matrix filled with the default value of the type `K`.
    ///
    /// If one of the parameters is equal to 0, returns `None`.
    ///
    /// # Example
    /// ```
    /// use matrix::Matrix;
    ///
    /// let mat: Matrix<f64> = Matrix::fill(2.5, 3, 2).unwrap();
    /// assert_eq!(mat, [[2.5, 2.5, 2.5], [2.5, 2.5, 2.5]]);
    /// let mat = Matrix::fill(-5, 1, 2).unwrap();
    /// assert_eq!(mat, [[-5], [-5]]);
    /// ```
    ///
    pub fn fill(default: K, number_of_column: usize, number_of_line: usize) -> Option<Self> {
        if number_of_column == 0 || number_of_line == 0 {
            return None;
        }
        let total_size = number_of_column * number_of_line;
        let mut content: Vec<K> = Vec::with_capacity(total_size);
        content.resize(total_size, default);
        Some(Self {
            content,
            dimensions: Dimensions {
                width: number_of_column,
                height: number_of_line,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use crate::Matrix;
    use pretty_assertions::assert_eq;

    #[test]
    fn columns_iter() {
        {
            let mat = Matrix::from([[1, 2], [3, 4]]);
            let mut iter = mat.columns();
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&4));
            assert_eq!(iter.next(), None);
        }
        {
            let mut mat = Matrix::from([[1, 2], [3, 4]]);
            let mut iter = mat.columns_mut();
            *iter.next().unwrap() = 1;
            *iter.next().unwrap() = 2;
            *iter.next().unwrap() = 3;
            *iter.next().unwrap() = 4;
            let mut iter = mat.columns();
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&4));
            assert_eq!(iter.next(), None);
        }
    }

    #[test]
    fn append_column() {
        let mut mat = Matrix::from([[1, 2], [4, 5]]);
        mat.append_column(&[3, 6]);
        assert_eq!(mat, [[1, 2, 3], [4, 5, 6]]);
    }

    #[test]
    fn swap_line() {
        {
            let mut mat: Matrix<u64> = Matrix::identity(&1, 3).unwrap();
            mat.swap_line(0, 2);
            assert_eq!(mat, [[0, 0, 1], [0, 1, 0], [1, 0, 0]]);
        }
        {
            let mut mat: Matrix<u64> = Matrix::identity(&1, 3).unwrap();
            mat.swap_line(2, 1);
            assert_eq!(mat, [[1, 0, 0], [0, 0, 1], [0, 1, 0]]);
        }
    }

    #[test]
    fn submatrix() {
        let matrix = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        {
            let submat = matrix.submatrix(0..2, 0..2).unwrap();
            assert_eq!(submat, [[1, 2], [4, 5]]);
        }
        {
            let submat = matrix.submatrix(1..3, 1..3).unwrap();
            assert_eq!(submat, [[5, 6], [8, 9]]);
        }
        {
            let submat = matrix.submatrix(0..1, 0..3).unwrap();
            assert_eq!(submat, [[1], [4], [7]]);
        }
    }
}
