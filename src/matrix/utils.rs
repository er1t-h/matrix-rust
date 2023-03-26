use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::{traits::Space, Matrix, Vector};

use self::iterator::{MatrixColumnIterator, MatrixColumnIteratorMut};
use super::Dimensions;

pub mod iterator;

// Display
impl<K: Space + Display> Display for Matrix<K> {
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
                .reduce(|accumulator, elt| accumulator + &", " + &elt)
                .unwrap();
            str += "], ";
        }
        write!(f, "[{}]", &str[..str.len() - 2])
    }
}

// From [[K]]
impl<K: Space, const LINE_SIZE: usize, const COLUMN_SIZE: usize> From<[[K; COLUMN_SIZE]; LINE_SIZE]>
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
impl<K: Space + Copy, const LINE_SIZE: usize> From<&[[K; LINE_SIZE]]> for Matrix<K> {
    #[inline(always)]
    fn from(value: &[[K; LINE_SIZE]]) -> Self {
        Self {
            content: value.into_iter().flatten().map(|x| *x).collect(),
            dimensions: Dimensions {
                width: value.len(),
                height: LINE_SIZE,
            },
        }
    }
}
impl<K: Space + Copy> From<&[&[K]]> for Matrix<K> {
    #[inline(always)]
    fn from(value: &[&[K]]) -> Self {
        Self {
            content: value.into_iter().flat_map(|x| *x).map(|x| *x).collect(),
            dimensions: Dimensions {
                width: value.len(),
                height: value.get(0).and_then(|x| Some(x.len())).unwrap_or(0),
            },
        }
    }
}
impl<K: Space> From<Vector<K>> for Matrix<K> {
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
impl<K: Space> IntoIterator for Matrix<K> {
    type Item = K;
    type IntoIter = <Vec<K> as IntoIterator>::IntoIter;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}
impl<'a, K: Space> IntoIterator for &'a Matrix<K> {
    type Item = &'a K;
    type IntoIter = <&'a Vec<K> as IntoIterator>::IntoIter;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.content.iter()
    }
}
impl<'a, K: Space> IntoIterator for &'a mut Matrix<K> {
    type Item = &'a mut K;
    type IntoIter = <&'a mut Vec<K> as IntoIterator>::IntoIter;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.content.iter_mut()
    }
}

// Derefs
impl<K: Space> Deref for Matrix<K> {
    type Target = [K];
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<K: Space> DerefMut for Matrix<K> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl<K: Space, const LINE_SIZE: usize, const COLUMN_SIZE: usize>
    PartialEq<[[K; COLUMN_SIZE]; LINE_SIZE]> for Matrix<K>
{
    fn eq(&self, other: &[[K; COLUMN_SIZE]; LINE_SIZE]) -> bool {
        Iterator::eq(self.iter(), other.into_iter().flatten())
    }
}

// Utils
impl<'a, K: Space> Matrix<K> {
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
    pub fn get_mut(&'a mut self, line: usize, column: usize) -> Option<&'a mut K> {
        if column < self.dimensions.width {
            self.content.get_mut(line * self.dimensions.width + column)
        } else {
            None
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
    pub fn columns(&'a self) -> MatrixColumnIterator<'a, K> {
        MatrixColumnIterator::new(self)
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
    #[inline(always)]
    pub fn columns_mut(&'a mut self) -> MatrixColumnIteratorMut<'a, K> {
        MatrixColumnIteratorMut::new(self)
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
}
