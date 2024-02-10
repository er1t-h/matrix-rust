use std::ops::Neg;

use crate::ConstVector;

impl<T: Neg<Output = R>, R, const SIZE: usize> Neg for ConstVector<T, SIZE> {
    type Output = ConstVector<R, SIZE>;
    fn neg(self) -> Self::Output {
        ConstVector {
            content: self.content.map(|x| -x),
        }
    }
}
