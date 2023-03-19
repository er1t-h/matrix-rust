use crate::{Vector, traits::Space, error::VectorOperationError};

impl <'a, K: Space> Vector<K>  {
	///
	/// Adds another `Vector` to self.
	/// If the size of the two Vectors differ, a `VectorOperationError` is returned.
	///
	/// # Example:
	/// ```
	/// use matrix::Vector;
	///
	/// let mut lhs = Vector::from([15, 2]);
	/// let rhs = Vector::from([3, 57]);
	/// assert_eq!(lhs.add_assign(&rhs), Ok(()));
	/// assert_eq!(lhs, [18, 59])
	/// ```
	///
	/// # Complexity:
	/// Linear in the `size` of the `Vectors`.
	///
	pub fn add_assign(&mut self, rhs: &Self) -> Result<(), VectorOperationError> {
		if self.size() != rhs.size() {
			return Err(VectorOperationError::NotSameSize(self.size(), rhs.size()));
		}
		for (lhs, rhs) in self.content.iter_mut().zip(rhs.content.iter()) {
			*lhs += rhs;
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{Vector, error::VectorOperationError};

	#[test]
	fn add_assign() {
		{
			let mut lhs = Vector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
			let rhs = Vector::from([9, 8, 7, 6, 5, 4, 3, 2, 1]);
			let trash = Vector::from([10, 2]);
			assert_eq!(lhs.add_assign(&rhs), Ok(()));
			assert_eq!(lhs, [10; 9]);
			assert_eq!(lhs.add_assign(&rhs), Ok(()));
			assert_eq!(lhs, [19, 18, 17, 16, 15, 14, 13, 12, 11]);
			assert_eq!(lhs.add_assign(&trash), Err(VectorOperationError::NotSameSize(9, 2)))
		}
		{
			let mut vec1 = Vector::from([9, 2, 5]);
			let mut vec2 = Vector::from([1, 6, -3]);
			let vec3 = Vector::from([8, 2, 21]);
			assert_eq!(vec2.add_assign(&vec3), Ok(()));
			assert_eq!(vec2, [9, 8, 18]);
			assert_eq!(vec1.add_assign(&vec2), Ok(()));
			assert_eq!(vec1, [18, 10, 23]);
		}
	}
}
