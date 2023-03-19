mod utils;
mod exercise_1;

use crate::traits::Space;


#[derive(Debug)]
pub struct Vector<K: Space> {
    content: Vec<K>,
}
