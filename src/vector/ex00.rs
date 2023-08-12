use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{
    error::VectorOperationError,
    traits::{SafeAdd, SafeSub},
    Vector,
};
// -----------------------------------------------------------------------------
// ------------------------------- ADD TRAITS
//
impl<K> SafeAdd for Vector<K>
where
    for<'a> K: Clone + AddAssign<K>,
{
    type Error = VectorOperationError;
    ///
    /// Adds another `Vector` to self.
    /// If the size of the two Vectors differ, a [`VectorOperationError`] is returned.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    /// use matrix::traits::SafeAdd;
    ///
    /// let mut lhs = Vector::from([15, 2]);
    /// let rhs = Vector::from([3, 57]);
    /// assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [18, 59])
    /// ```
    ///
    /// # Errors
    /// If the size of `self` and `rhs` don't match, return a [`NotSameSize`](VectorOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear in the `size` of the `Vectors`.
    ///
    fn safe_add_assign(&mut self, rhs: Self) -> Result<(), Self::Error> {
        if self.size() != rhs.size() {
            return Err(VectorOperationError::NotSameSize(self.size(), rhs.size()));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content) {
            *lhs += rhs;
        }
        Ok(())
    }
}

impl<K> SafeAdd<&Self> for Vector<K>
where
    for<'a> K: Clone + AddAssign<&'a K>,
{
    type Error = VectorOperationError;
    ///
    /// Adds another `Vector` to self.
    /// If the size of the two Vectors differ, a [`VectorOperationError`] is returned.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    /// use matrix::traits::SafeAdd;
    ///
    /// let mut lhs = Vector::from([15, 2]);
    /// let rhs = Vector::from([3, 57]);
    /// assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [18, 59])
    /// ```
    ///
    /// # Errors
    /// If the size of `self` and `rhs` don't match, return a [`NotSameSize`](VectorOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear in the `size` of the `Vectors`.
    ///
    fn safe_add_assign(&mut self, rhs: &Self) -> Result<(), Self::Error> {
        if self.size() != rhs.size() {
            return Err(VectorOperationError::NotSameSize(self.size(), rhs.size()));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content.iter()) {
            *lhs += rhs;
        }
        Ok(())
    }
}

impl<K> AddAssign<&Self> for Vector<K>
where
    for<'a> K: Clone + AddAssign<&'a K>,
{
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        let _ = self.safe_add_assign(rhs);
    }
}

impl<K> AddAssign for Vector<K>
where
    for<'a> K: Clone + AddAssign<K>,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let _ = self.safe_add_assign(rhs);
    }
}

impl<K> Add<&Self> for Vector<K>
where
    for<'a> K: Clone + AddAssign<&'a K>,
{
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<K> Add for Vector<K>
where
    for<'a> K: Clone + AddAssign<K>,
{
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

// -----------------------------------------------------------------------------
// ------------------------------- SUB TRAITS
//

impl<K> SafeSub for Vector<K>
where
    for<'a> K: Clone + SubAssign<K>,
{
    type Error = VectorOperationError;
    ///
    /// Substracts another `Vector` from self.
    /// If the size of the two Vectors differ, a [`VectorOperationError`] is returned.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    /// use matrix::traits::SafeSub;
    ///
    /// let mut lhs = Vector::from([15, 2]);
    /// let rhs = Vector::from([3, 57]);
    /// assert_eq!(lhs.safe_sub_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [12, -55])
    /// ```
    ///
    /// # Errors
    /// If the size of `self` and `rhs` don't match, return a [`NotSameSize`](VectorOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear in the `size` of the `Vectors`.
    ///
    fn safe_sub_assign(&mut self, rhs: Self) -> Result<(), VectorOperationError> {
        if self.size() != rhs.size() {
            return Err(VectorOperationError::NotSameSize(self.size(), rhs.size()));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content) {
            *lhs -= rhs;
        }
        Ok(())
    }
}

impl<K> SafeSub<&Self> for Vector<K>
where
    for<'a> K: Clone + SubAssign<&'a K>,
{
    type Error = VectorOperationError;
    ///
    /// Substracts another `Vector` from self.
    /// If the size of the two Vectors differ, a [`VectorOperationError`] is returned.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    /// use matrix::traits::SafeSub;
    ///
    /// let mut lhs = Vector::from([15, 2]);
    /// let rhs = Vector::from([3, 57]);
    /// assert_eq!(lhs.safe_sub_assign(&rhs), Ok(()));
    /// assert_eq!(lhs, [12, -55])
    /// ```
    ///
    /// # Errors
    /// If the size of `self` and `rhs` don't match, return a [`NotSameSize`](VectorOperationError::NotSameSize)
    ///
    /// # Complexity:
    /// Linear in the `size` of the `Vectors`.
    ///
    fn safe_sub_assign(&mut self, rhs: &Self) -> Result<(), VectorOperationError> {
        if self.size() != rhs.size() {
            return Err(VectorOperationError::NotSameSize(self.size(), rhs.size()));
        }
        for (lhs, rhs) in self.content.iter_mut().zip(rhs.content.iter()) {
            *lhs -= rhs;
        }
        Ok(())
    }
}

impl<K> SubAssign<&Self> for Vector<K>
where
    for<'a> K: Clone + SubAssign<&'a K>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        let _ = self.safe_sub_assign(rhs);
    }
}

impl<K> SubAssign for Vector<K>
where
    for<'a> K: Clone + SubAssign<K>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let _ = self.safe_sub_assign(rhs);
    }
}

impl<K> Sub<&Self> for Vector<K>
where
    for<'a> K: Clone + SubAssign<&'a K>,
{
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<K> Sub for Vector<K>
where
    for<'a> K: Clone + SubAssign<K>,
{
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

// -----------------------------------------------------------------------------
// ------------------------------- MUL TRAITS
//

impl<K> MulAssign<&K> for Vector<K>
where
    for<'a> K: Clone + MulAssign<&'a K>,
{
    ///
    /// Multiplies a scalar into self.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    ///
    /// let mut lhs = Vector::from([15, 2]);
    /// lhs *= 3;
    /// assert_eq!(lhs, [45, 6])
    /// ```
    ///
    /// # Complexity:
    /// Linear in the `size` of the `self`.
    ///
    #[inline]
    fn mul_assign(&mut self, rhs: &K) {
        for elt in &mut self.content {
            *elt *= rhs;
        }
    }
}

impl<K> MulAssign<K> for Vector<K>
where
    for<'a> K: Clone + MulAssign<K>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: K) {
        for elt in &mut self.content {
            *elt *= rhs.clone();
        }
    }
}

impl<K> Mul<&K> for Vector<K>
where
    for<'a> K: Clone + MulAssign<&'a K>,
{
    type Output = Self;
    #[inline]
    fn mul(mut self, rhs: &K) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<K> Mul<K> for Vector<K>
where
    for<'a> K: Clone + MulAssign<K>,
{
    type Output = Self;
    #[inline]
    fn mul(mut self, rhs: K) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<K> Mul<&K> for &Vector<K>
where
    for<'a> K: Clone + MulAssign<&'a K>,
{
    type Output = Vector<K>;
    #[inline]
    fn mul(self, rhs: &K) -> Self::Output {
        let mut ret = self.clone();
        ret *= rhs;
        ret
    }
}

impl<K> Mul<K> for &Vector<K>
where
    for<'a> K: Clone + MulAssign<K>,
{
    type Output = Vector<K>;
    #[inline]
    fn mul(self, rhs: K) -> Self::Output {
        let mut ret = self.clone();
        ret *= rhs;
        ret
    }
}

// -----------------------------------------------------------------------------
// ------------------------------- TESTS
//

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{
        complex::cpl,
        error::VectorOperationError,
        traits::{SafeAdd, SafeSub},
        Vector,
    };

    #[test]
    fn safe_add_assign() {
        {
            let mut lhs = Vector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let rhs = Vector::from([9, 8, 7, 6, 5, 4, 3, 2, 1]);
            let trash = Vector::from([10, 2]);
            assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
            assert_eq!(lhs, [10; 9]);
            assert_eq!(lhs.safe_add_assign(&rhs), Ok(()));
            assert_eq!(lhs, [19, 18, 17, 16, 15, 14, 13, 12, 11]);
            assert_eq!(
                lhs.safe_add_assign(&trash),
                Err(VectorOperationError::NotSameSize(9, 2))
            );
        }
        {
            let mut vec1 = Vector::from([9, 2, 5]);
            let mut vec2 = Vector::from([1, 6, -3]);
            let vec3 = Vector::from([8, 2, 21]);
            assert_eq!(vec2.safe_add_assign(&vec3), Ok(()));
            assert_eq!(vec2, [9, 8, 18]);
            assert_eq!(vec1.safe_add_assign(&vec2), Ok(()));
            assert_eq!(vec1, [18, 10, 23]);
        }
    }

    #[test]
    fn add_assign() {
        {
            let mut lhs = Vector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let rhs = Vector::from([9, 8, 7, 6, 5, 4, 3, 2, 1]);
            let trash = Vector::from([10, 2]);
            lhs += &rhs;
            assert_eq!(lhs, [10; 9]);
            lhs += &rhs;
            assert_eq!(lhs, [19, 18, 17, 16, 15, 14, 13, 12, 11]);
            lhs += &trash;
            assert_eq!(lhs, [19, 18, 17, 16, 15, 14, 13, 12, 11]);
        }
        {
            let mut vec1 = Vector::from([9, 2, 5]);
            let mut vec2 = Vector::from([1, 6, -3]);
            let vec3 = Vector::from([8, 2, 21]);
            vec2 += &vec3;
            assert_eq!(vec2, [9, 8, 18]);
            vec1 += &vec2;
            assert_eq!(vec1, [18, 10, 23]);
        }
    }

    #[test]
    fn add() {
        let vec1 = Vector::from([1, 2, 3]);
        assert_eq!(vec1.clone() + &vec1 + &vec1 + &vec1, [4, 8, 12]);
        assert_eq!(vec1, [1, 2, 3]);
    }

    #[test]
    fn safe_sub_assign() {
        {
            let mut lhs = Vector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let rhs = Vector::from([9, 8, 7, 6, 5, 4, 3, 2, 1]);
            let trash = Vector::from([10, 2]);
            assert_eq!(lhs.safe_sub_assign(&rhs), Ok(()));
            assert_eq!(lhs, [-8, -6, -4, -2, 0, 2, 4, 6, 8]);
            assert_eq!(lhs.safe_sub_assign(&rhs), Ok(()));
            assert_eq!(lhs, [-17, -14, -11, -8, -5, -2, 1, 4, 7]);
            assert_eq!(
                lhs.safe_sub_assign(&trash),
                Err(VectorOperationError::NotSameSize(9, 2))
            );
        }
        {
            let mut vec1 = Vector::from([9, 2, 5]);
            let mut vec2 = Vector::from([1, 6, -3]);
            let vec3 = Vector::from([8, 2, 21]);
            assert_eq!(vec2.safe_sub_assign(&vec3), Ok(()));
            assert_eq!(vec2, [-7, 4, -24]);
            assert_eq!(vec1.safe_sub_assign(&vec2), Ok(()));
            assert_eq!(vec1, [16, -2, 29]);
        }
    }

    #[test]
    fn sub_assign() {
        let mut lhs = Vector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let rhs = Vector::from([9, 8, 7, 6, 5, 4, 3, 2, 1]);
        lhs -= &rhs;
        assert_eq!(lhs, [-8, -6, -4, -2, 0, 2, 4, 6, 8]);
    }

    #[test]
    fn sub() {
        let vec1 = Vector::from([1, 2, 3]);
        assert_eq!(
            vec1.clone() - vec1.clone() - vec1.clone() - &vec1,
            [-2, -4, -6]
        );
        assert_eq!(vec1, [1, 2, 3]);
    }

    #[test]
    fn mul_assign() {
        let mut vec = Vector::from([1, 5, 8, 4]);
        vec *= 4;
        assert_eq!(vec, [4, 20, 32, 16]);
    }

    #[test]
    fn example() {
        {
            let mut u = Vector::from([2., 3.]);
            let v = Vector::from([5., 7.]);
            u += v;
            println!("{u}");
            assert_eq!(u, [7., 10.]);
        }
        {
            let mut u = Vector::from([2., 3.]);
            let v = Vector::from([5., 7.]);
            u -= v;
            println!("{u}");
            assert_eq!(u, [-3., -4.]);
        }
        {
            let mut u = Vector::from([2., 3.]);
            u *= 2.;
            println!("{u}");
            assert_eq!(u, [4., 6.]);
        }
    }

    #[test]
    fn add_with_complex() {
        let u = Vector::from([cpl!(1 + 2 i), cpl!(3 + 4 i)]);
        let v = Vector::from([cpl!(1 + 2 i), cpl!(3 + 4 i)]);
        assert_eq!(u + v, [cpl!(2 + 4 i), cpl!(6 + 8 i)]);
    }

    #[test]
    fn sub_with_complex() {
        let u = Vector::from([cpl!(1 + 2 i), cpl!(3 + 4 i)]);
        let v = Vector::from([cpl!(1 + 2 i), cpl!(3 + 4 i)]);
        assert_eq!(u - v, [cpl!(0 + 0 i), cpl!(0 + 0 i)]);
    }

    #[test]
    fn mul_with_complex() {
        let u = Vector::from([cpl!(1 + 2 i), cpl!(3 + 4 i)]);
        assert_eq!(u * cpl!(5 + 2 i), [cpl!(1 + 12 i), cpl!(7 + 26 i)]);
    }
}
