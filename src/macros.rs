#[macro_export]
macro_rules! mat {
	[$($elt: expr),+] => {
		$crate::matrix::Matrix::from([$($elt),*])
	};
}
