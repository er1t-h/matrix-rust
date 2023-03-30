use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    slice::{Iter, IterMut},
};

use crate::{Matrix, Vector};

use self::{
    columns::{MatrixColumn, MatrixColumnMut},
    iterator::{MatrixIterByColumn, MatrixIterByColumnMut},
};
use super::Dimensions;

pub mod columns;
pub mod iterator;

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
                .map(|x| x.to_string())
                .reduce(|accumulator, elt| accumulator + ", " + &elt)
                .unwrap();
            str += "], ";
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
impl<K: Clone + Copy, const LINE_SIZE: usize> From<&[[K; LINE_SIZE]]> for Matrix<K> {
    #[inline(always)]
    fn from(value: &[[K; LINE_SIZE]]) -> Self {
        Self {
            content: value.iter().flatten().copied().collect(),
            dimensions: Dimensions {
                width: value.len(),
                height: LINE_SIZE,
            },
        }
    }
}
impl<K: Clone + Copy> From<&[&[K]]> for Matrix<K> {
    #[inline(always)]
    fn from(value: &[&[K]]) -> Self {
        Self {
            content: value.iter().flat_map(|x| *x).copied().collect(),
            dimensions: Dimensions {
                width: value.len(),
                height: value.get(0).map(|x| x.len()).unwrap_or(0),
            },
        }
    }
}
impl<K: Clone> From<Vector<K>> for Matrix<K> {
    #[inline(always)]
    fn from(value: Vector<K>) -> Self {
        let len = value.len();
        Matrix {
            content: value.into_iter().collect(),
            dimensions: Dimensions {
                width: 1,
                height: len,
            },
        }
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
    pub fn size(&self) -> (usize, usize) {
        (self.dimensions.height, self.dimensions.width)
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
    pub fn is_square(&self) -> bool {
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
    pub fn get_line(&'a self, line_number: usize) -> Option<Iter<'a, K>> {
        if self.dimensions.height < line_number || self.dimensions.width == 0 {
            None
        } else {
            let begin_line = self.dimensions.width * line_number;
            Some(self.content[begin_line..begin_line + self.dimensions.width].iter())
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
    pub fn columns(&'a self) -> MatrixIterByColumn<'a, K> {
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
    // use pretty_assertions::assert_eq;

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
}
