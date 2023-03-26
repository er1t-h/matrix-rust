use std::{
    fmt::Display,
    ops::{Deref, DerefMut, Index},
    slice::SliceIndex,
};
// mod iterator;

use super::Vector;

impl<K: Clone + Display> Display for Vector<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.content.is_empty() {
            let buff = self
                .content
                .iter()
                .map(|x| x.to_string())
                .reduce(|accumulator, elt| accumulator + ", " + &elt)
                .unwrap();
            write!(f, "[{}]", buff)
        } else {
            write!(f, "[]")
        }
    }
}

impl<K: Clone> From<&[K]> for Vector<K> {
    #[inline(always)]
    fn from(base: &[K]) -> Self {
        Self {
            content: Vec::from(base),
        }
    }
}

impl<K: Clone, const SIZE: usize> From<[K; SIZE]> for Vector<K> {
    #[inline(always)]
    fn from(base: [K; SIZE]) -> Self {
        Self {
            content: Vec::from(base),
        }
    }
}

impl<K, const SIZE: usize> PartialEq<[K; SIZE]> for Vector<K>
where
    K: PartialEq + Clone,
{
    #[inline(always)]
    fn eq(&self, other: &[K; SIZE]) -> bool {
        self.content == other
    }
}

impl<K> PartialEq<&[K]> for Vector<K>
where
    K: PartialEq + Clone,
{
    #[inline(always)]
    fn eq(&self, other: &&[K]) -> bool {
        &self.content == other
    }
}

impl<K: Clone, Idx> Index<Idx> for Vector<K>
where
    Idx: SliceIndex<[K], Output = K>,
{
    type Output = K;

    #[inline(always)]
    fn index(&self, index: Idx) -> &Self::Output {
        self.content.index(index)
    }
}

impl<K: Clone> IntoIterator for Vector<K> {
    type Item = K;
    type IntoIter = <Vec<K> as IntoIterator>::IntoIter;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}
impl<'a, K: Clone> IntoIterator for &'a Vector<K> {
    type Item = &'a K;
    type IntoIter = <&'a Vec<K> as IntoIterator>::IntoIter;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.content.iter()
    }
}
impl<'a, K: Clone> IntoIterator for &'a mut Vector<K> {
    type Item = &'a mut K;
    type IntoIter = <&'a mut Vec<K> as IntoIterator>::IntoIter;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.content.iter_mut()
    }
}
impl<K: Clone> Deref for Vector<K> {
    type Target = [K];
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<K: Clone> DerefMut for Vector<K> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl<K: Clone> Vector<K> {
    ///
    /// Returns the `size` of the `Vector`
    ///
    /// # Example
    /// ```
    /// use matrix::Vector;
    ///
    /// let vec = Vector::from([1, 2, 3]);
    /// assert_eq!(vec.size(), 3);
    /// ```
    ///
    #[inline(always)]
    pub fn size(&self) -> usize {
        self.content.len()
    }

    ///
    /// Returns one or more reference to the objects of the `Vector`, or `None`
    /// if it's outside bounds the `index` passed is outside bounds
    ///
    /// # Example
    /// ```
    /// use matrix::Vector;
    ///
    /// let vec = Vector::from([1, 2, 3]);
    /// assert_eq!(vec.get(2), Some(&3));
    /// assert_eq!(vec.get(4), None);
    /// ```
    ///
    #[inline(always)]
    pub fn get<Idx: SliceIndex<[K], Output = K>>(&self, index: Idx) -> Option<&K> {
        self.content.get(index)
    }

    ///
    /// Returns one or more reference to the objects of the `Vector`, or `None`
    /// if it's outside bounds the `index` passed is outside bounds
    ///
    /// # Example
    /// ```
    /// use matrix::Vector;
    ///
    /// let mut vec = Vector::from([1, 2, 3]);
    /// let index_0 = vec.get_mut(0).unwrap();
    /// *index_0 = 6;
    /// assert_eq!(vec.get(0), Some(&6));
    /// ```
    ///
    #[inline(always)]
    pub fn get_mut<Idx: SliceIndex<[K], Output = K>>(&mut self, index: Idx) -> Option<&mut K> {
        self.content.get_mut(index)
    }

    ///
    /// Adds another item at the end of the `Vector`
    ///
    /// # Example
    /// ```
    /// use matrix::Vector;
    ///
    /// let mut vec = Vector::from([1, 2]);
    /// vec.append(3);
    /// assert_eq!(vec, [1, 2, 3]);
    /// ```
    #[inline(always)]
    pub fn append(&mut self, number: K) {
        self.content.push(number);
    }

    pub fn new() -> Self {
        Vector {
            content: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Vector;
    use pretty_assertions::assert_eq;

    #[test]
    fn display() {
        let test = Vector {
            content: vec![15, 1, -7],
        };
        assert_eq!(format!("{}", test), "[15, 1, -7]");
        let test = Vector { content: vec![15] };
        assert_eq!(format!("{}", test), "[15]");
        let test: Vector<u64> = Vector {
            content: Vec::new(),
        };
        assert_eq!(format!("{}", test), "[]");
    }

    #[test]
    fn getters() {
        let mut test = Vector {
            content: vec![15, 1, -7],
        };
        assert_eq!(test.get(0), Some(&15));
        assert_eq!(test.get(1), Some(&1));
        assert_eq!(test.get(2), Some(&-7));
        assert_eq!(test.get(3), None);
        *test.get_mut(0).unwrap() = 55;
        assert_eq!(test.get(0), Some(&55));
    }

    #[test]
    fn size() {
        let test = Vector {
            content: vec![15, 1, -7],
        };
        assert_eq!(test.size(), 3);
        let test: Vector<i64> = Vector { content: vec![] };
        assert_eq!(test.size(), 0);
        let test = Vector {
            content: vec![10, 5497, -45, 4454, 564, 51, 10, 0, 56],
        };
        assert_eq!(test.size(), 9);
    }

    #[test]
    fn from_tab() {
        let test = Vector::from([45, 454, 42, 48884, 33154]);
        assert_eq!(format!("{}", test), "[45, 454, 42, 48884, 33154]")
    }

    #[test]
    fn append() {
        let mut vec: Vector<u32> = Vector::new();
        vec.append(1);
        vec.append(2);
        vec.append(3);
        assert_eq!(vec, [1, 2, 3]);
    }

    #[test]
    fn iterators() {
        // Using iter()
        {
            let test = Vector::from([1, 2, 3, 4, 5_u64]);
            let mut accumulator = 0;
            for i in test.iter() {
                accumulator += i;
            }
            assert_eq!(accumulator, 15);
        }
        // Using iter_mut()
        {
            let mut test = Vector::from([1, 2, 3, 4, 5_u64]);
            for i in test.iter_mut() {
                *i = (*i).pow(2);
            }
            assert_eq!(test, [1, 4, 9, 16, 25]);
        }
        // Using IntoIter
        {
            let test = Vector::from([1, 2, 3, 4, 5_u64]);
            let mut accumulator = 0;
            for i in test {
                accumulator += i;
            }
            assert_eq!(accumulator, 15);
        }
        // Using IntoIter as reference
        {
            let test = Vector::from([1, 2, 3, 4, 5_u64]);
            let mut accumulator = 0;
            for i in &test {
                accumulator += i;
            }
            assert_eq!(accumulator, 15);
        }
        // Using IntoIter as mutable reference
        {
            let mut test = Vector::from([1, 2, 3, 4, 5_u64]);
            for i in &mut test {
                *i = (*i).pow(2);
            }
            assert_eq!(test, [1, 4, 9, 16, 25]);
        }
    }
}
