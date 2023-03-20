use std::ops::{Deref, DerefMut};

use crate::{traits::Space, Matrix};

use self::iterator::{MatrixColumnIterator, MatrixColumnIteratorMut};

pub mod iterator;

impl <K: Space, const LINE_SIZE: usize, const COLUMN_SIZE: usize> From<[[K; COLUMN_SIZE]; LINE_SIZE]> for Matrix<K> {
	fn from(value: [[K; COLUMN_SIZE]; LINE_SIZE]) -> Self {
		Self {
			content: value.into_iter().flatten().collect(),
			width: COLUMN_SIZE,
			height: LINE_SIZE
		}
	}
}
impl <K: Space + Copy, const LINE_SIZE: usize> From<&[[K; LINE_SIZE]]> for Matrix<K> {
	fn from(value: &[[K; LINE_SIZE]]) -> Self {
		Self {
			content: value.into_iter().flatten().map(|x| *x).collect(),
			width: value.len(),
			height: LINE_SIZE
		}
	}
}
impl <K: Space + Copy> From<&[&[K]]> for Matrix<K> {
	fn from(value: &[&[K]]) -> Self {
		Self {
			content: value.into_iter().flat_map(|x| *x).map(|x| *x).collect(),
			width: value.len(),
			height: value.get(0).and_then(|x| Some(x.len())).unwrap_or(0)
		}
	}
}

impl <K: Space> IntoIterator for Matrix<K> {
    type Item = K;
    type IntoIter = <Vec<K> as IntoIterator>::IntoIter;
	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.content.into_iter()
	}
}
impl <'a, K: Space> IntoIterator for &'a Matrix<K> {
	type Item = &'a K;
	type IntoIter = <&'a Vec<K> as IntoIterator>::IntoIter;
	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.content.iter()
	}
}
impl <'a, K: Space> IntoIterator for &'a mut Matrix<K> {
	type Item = &'a mut K;
	type IntoIter = <&'a mut Vec<K> as IntoIterator>::IntoIter;
	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.content.iter_mut()
	}
}
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

impl <'a, K: Space> Matrix<K> {
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
		(self.height, self.width)
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
		self.height == self.width
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
		if column < self.width {
			self.content.get(line * self.width + column)
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
		if column < self.width {
			self.content.get_mut(line * self.width + column)
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

	#[test]
	fn from() {
		{
			let mat1 = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
			println!("{:?}", mat1);
		}
	}

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
