use std::ops::MulAssign;

use crate::{const_vector::ConstVector, traits::TermToTerm};

impl<K, const SIZE: usize> TermToTerm for ConstVector<K, SIZE>
where
    K: MulAssign,
{
    fn term_to_term_mul_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content) {
            *lhs *= rhs;
        }
    }
}

impl<K, const SIZE: usize> TermToTerm<&Self> for ConstVector<K, SIZE>
where
    for<'a> K: MulAssign<&'a K>,
{
    fn term_to_term_mul_assign(&mut self, rhs: &Self) {
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content.iter()) {
            *lhs *= rhs;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{const_vector::ConstVector, traits::TermToTerm};

    #[test]
    fn mul_assign() {
        let mut vec1 = ConstVector::from([1, 4, 2]);
        let vec2 = ConstVector::from([2, 3, 7]);
        vec1.term_to_term_mul_assign(vec2);
        assert_eq!(vec1, ConstVector::from([2, 12, 14]));
    }

    #[test]
    fn mul_assign_ref() {
        let mut vec1 = ConstVector::from([1, 4, 2]);
        let vec2 = ConstVector::from([2, 3, 7]);
        vec1.term_to_term_mul_assign(&vec2);
        assert_eq!(vec1, ConstVector::from([2, 12, 14]));
        assert_eq!(vec2, ConstVector::from([2, 3, 7]));
    }

    #[test]
    fn mul() {
        let vec1 = ConstVector::from([1, 4, 2]);
        let vec2 = ConstVector::from([2, 3, 7]);
        assert_eq!(vec1.term_to_term_mul(vec2), ConstVector::from([2, 12, 14]));
    }

    #[test]
    fn mul_ref() {
        let vec1 = ConstVector::from([1, 4, 2]);
        let vec2 = ConstVector::from([2, 3, 7]);
        assert_eq!(vec1.term_to_term_mul(&vec2), ConstVector::from([2, 12, 14]));
        assert_eq!(vec2, ConstVector::from([2, 3, 7]));
    }
}
