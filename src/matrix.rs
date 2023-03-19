mod ex00;
mod utils;
use crate::traits::Space;

#[derive(Debug, Clone)]
pub struct Matrix<K: Space> {
    content: Vec<K>,
	width: usize,
	height: usize
}
