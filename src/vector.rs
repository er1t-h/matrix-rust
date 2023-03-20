mod ex00;
mod utils;

use crate::traits::Space;

///
/// Implementation of a Vector
///
#[derive(Debug, Clone)]
pub struct Vector<K: Space> {
    content: Vec<K>,
}
