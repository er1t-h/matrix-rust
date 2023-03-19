use std::{
    fmt::Display,
};

use crate::traits::Space;
use super::Vector;

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

impl <K: Space, const SIZE: usize> From<[K; SIZE]> for Vector<K> {
    fn from(base: [K; SIZE]) -> Self {
        Self {
            content: Vec::from(base)
        }
    }
}

impl <K: Space, const SIZE: usize> PartialEq<[K; SIZE]> for Vector<K> {
    fn eq(&self, other: &[K; SIZE]) -> bool {
        self.content == other
    }
}

impl<K: Space> Vector<K> {
    pub fn size(&self) -> usize {
        self.content.len()
    }

    pub fn get(&self, index: usize) -> Option<&K> {
        self.content.get(index)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use super::Vector;

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
    fn get() {
        let test = Vector {
            content: vec![15, 1, -7],
        };
        assert_eq!(test.get(0), Some(&15));
        assert_eq!(test.get(1), Some(&1));
        assert_eq!(test.get(2), Some(&-7));
        assert_eq!(test.get(3), None);
    }

    #[test]
    fn size() {
        let test = Vector {
            content: vec![15, 1, -7],
        };
        assert_eq!(test.size(), 3);
        let test: Vector<i64> = Vector {
            content: vec![],
        };
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
