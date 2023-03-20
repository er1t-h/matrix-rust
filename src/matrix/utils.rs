use crate::{traits::Space, Matrix};

impl <K: Space, const LINE_SIZE: usize, const COLUMN_SIZE: usize> From<[[K; LINE_SIZE]; COLUMN_SIZE]> for Matrix<K> {
	fn from(value: [[K; LINE_SIZE]; COLUMN_SIZE]) -> Self {
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

impl <K: Space> Matrix<K> {
	pub fn size(&self) -> (usize, usize) {
		(self.height, self.width)
	}

	pub fn is_square(&self) -> bool {
		self.height == self.width
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
}
