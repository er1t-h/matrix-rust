use std::ops::Index;

use crate::static_asserts::{AssertNonZero, AssertNonZeroSizeType};

mod operations;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ConstVector<K, const SIZE: usize> {
    content: [K; SIZE],
}

impl<K, const SIZE: usize> From<[K; SIZE]> for ConstVector<K, SIZE> {
    #[allow(clippy::no_effect, path_statements)]
    fn from(vector: [K; SIZE]) -> Self {
        AssertNonZero::<SIZE>::OK;
        AssertNonZeroSizeType::<K>::OK;

        Self { content: vector }
    }
}

impl<K, const SIZE: usize> Index<usize> for ConstVector<K, SIZE> {
    type Output = K;
    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}

pub type Vec2<T = f32> = ConstVector<T, 2>;
pub type Vec3<T = f32> = ConstVector<T, 3>;
pub type Vec4<T = f32> = ConstVector<T, 4>;

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { content: [x, y] }
    }
}

impl<T> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { content: [x, y, z] }
    }
}

impl<T> Vec4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            content: [x, y, z, w],
        }
    }
}
