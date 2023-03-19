use std::{fmt::Display, ops::Index, slice::SliceIndex};

use super::Vector;
use crate::traits::Space;

impl<K: Space + Display> Display for Vector<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.content.is_empty() {
            let buff = self
                .content
                .iter()
                .map(|x| x.to_string())
                .reduce(|accumulator, elt| accumulator + &", " + &elt)
                .unwrap();
            write!(f, "[{}]", buff)
        } else {
            write!(f, "[]")
        }
    }
}

impl<K: Space> From<&[K]> for Vector<K> {
    #[inline]
    fn from(base: &[K]) -> Self {
        Self {
            content: Vec::from(base),
        }
    }
}

impl<K: Space, const SIZE: usize> From<[K; SIZE]> for Vector<K> {
    #[inline]
    fn from(base: [K; SIZE]) -> Self {
        Self {
            content: Vec::from(base),
        }
    }
}

impl<K: Space, const SIZE: usize> PartialEq<[K; SIZE]> for Vector<K> {
    #[inline]
    fn eq(&self, other: &[K; SIZE]) -> bool {
        &self.content == other
    }
}


impl<K: Space> PartialEq<&[K]> for Vector<K> {
    #[inline]
    fn eq(&self, other: &&[K]) -> bool {
        &self.content == other
    }
}

impl<K: Space, Idx> Index<Idx> for Vector<K>
where
    Idx: SliceIndex<[K], Output = K>,
{
    type Output = K;

    #[inline]
    fn index(&self, index: Idx) -> &Self::Output {
        self.content.index(index)
    }
}

impl<K: Space> Vector<K> {
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn get_mut<Idx: SliceIndex<[K], Output = K>>(&mut self, index: Idx) -> Option<&mut K> {
        self.content.get_mut(index)
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
}
