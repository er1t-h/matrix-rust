use crate::{traits::Space, Matrix, error::MatrixOperationError};

impl <K: Space> Matrix<K> {
	///
	/// Adds another `Matrix` to self
	/// If the size of the two Matrixes differ, a `MatrixOperationError` is returned
	///
	/// # Example:
	/// ```
	/// use matrix::Matrix;
	///
	/// let mut lhs = Matrix::from([[5, 4], [3, 2]]);
	/// let rhs = Matrix::from([[5, 6], [7, 8]]);
	/// assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
	/// assert_eq!(lhs, [[10, 10], [10, 10]])
	/// ```
	///
	pub fn safe_add_assign(&mut self, rhs: &Self) -> Result<(), MatrixOperationError> {
		if self.dimensions != rhs.dimensions {
			return Err(MatrixOperationError::NotSameSize(self.dimensions, rhs.dimensions));
		}
		for (lhs, rhs) in self.into_iter().zip(rhs.iter()) {
			*lhs += rhs;
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
    use crate::{Matrix, matrix::Dimensions};

	#[test]
	fn safe_add_assign() {
		{
			let mut lhs = Matrix::from([[1, 2, 3], [4, 5, 6]]);
			let rhs = Matrix::from([[6, 5, 4], [3, 2, 1]]);
			let trash = Matrix::from([[6, 5, 4, 3, 2, 1]]);

			assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
			assert_eq!(lhs, [[7;3], [7;3]]);
			assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
			assert_eq!(lhs, [[13, 12, 11], [10, 9, 8]]);
			assert_eq!(
				lhs.safe_add_assign(&trash),
				Err(crate::error::MatrixOperationError::NotSameSize(Dimensions { height: 2, width: 3}, Dimensions { width: 6, height: 1 }))
			)
		}
	}
}
