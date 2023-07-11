//!
//! Taxicab, Euclidean and Suprenum norms implementation
//!

use std::{iter::Sum, ops::Mul};

use crate::{
    traits::{self, Abs, Max, Sqrt, Zero},
    Vector,
};

impl<K> Vector<K>
where
    K: Clone + Abs + Sum,
{
    ///
    /// Returns the [`taxicab norm`](https://en.wikipedia.org/wiki/Taxicab_geometry)
    /// of a Vector.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    ///
    /// let vec = Vector::from([5, 2, 1]);
    /// assert_eq!(vec.norm_1(), 8);
    /// ```
    ///
    /// # Complexity:
    /// Linear: O(n) with `n` the size of the vector.
    ///
    #[must_use]
    pub fn norm_1(&self) -> K {
        self.content.iter().map(traits::Abs::abs).sum()
    }
}

impl<K> Vector<K>
where
    K: Clone + Sqrt + Sum,
    for<'a> &'a K: Mul<&'a K, Output = K>,
{
    ///
    /// Returns the [`euclidean norm`](https://en.wikipedia.org/wiki/Norm_(mathematics))
    /// of a Vector.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    ///
    /// let vec = Vector::from([4., 2., 2., 1.]);
    /// assert_eq!(vec.norm(), 5.);
    /// ```
    ///
    /// # Complexity:
    /// Linear: O(n) with `n` the size of the vector.
    ///
    #[must_use]
    pub fn norm(&self) -> K {
        self.content.iter().map(|x| x * x).sum::<K>().sqrt()
    }
}

impl<K> Vector<K>
where
    K: Clone + Zero + Abs + Max,
{
    ///
    /// Returns the [`supremum norm`](https://en.wikipedia.org/wiki/Uniform_norm)
    /// of a Vector.
    ///
    /// # Example:
    /// ```
    /// use matrix::Vector;
    ///
    /// let vec = Vector::from([5, 2, 1]);
    /// assert_eq!(vec.norm_inf(), 5);
    /// ```
    ///
    /// # Complexity:
    /// Linear: O(n) with `n` the size of the vector.
    ///
    pub fn norm_inf(&self) -> K {
        self.content
            .iter()
            .cloned()
            .map(|x| x.abs())
            .reduce(K::max)
            .unwrap_or_else(K::zero)
    }
}

#[cfg(test)]
mod test {
    use crate::{assert_eq_float, complex::cpl};
    use pretty_assertions::assert_eq;

    use crate::Vector;

    #[test]
    fn example() {
        {
            let u = Vector::from([0., 0., 0.]);
            {
                let res = u.norm_1();
                assert_eq!(res, 0.);
                println!("norm1({u}) = {res}");
            }
            {
                let res = u.norm();
                assert_eq!(res, 0.);
                println!("norm({u}) = {res}");
            }
            {
                let res = u.norm_inf();
                assert_eq!(res, 0.);
                println!("norm_inf({u}) = {res}");
            }
        }
        {
            let u = Vector::from([1., 2., 3.]);
            {
                let res = u.norm_1();
                assert_eq!(res, 6.);
                println!("norm1({u}) = {res}");
            }
            {
                let res = u.norm();
                assert_eq_float!(res, 3.741_657_38_f64);
                println!("norm({u}) = {res}");
            }
            {
                let res = u.norm_inf();
                assert_eq!(res, 3.);
                println!("norm_inf({u}) = {res}");
            }
        }
        {
            let u = Vector::from([-1., -2.]);
            {
                let res = u.norm_1();
                assert_eq!(res, 3.);
                println!("norm1({u}) = {res}");
            }
            {
                let res = u.norm();
                assert_eq_float!(res, 2.236_067_977_f64);
                println!("norm({u}) = {res}");
            }
            {
                let res = u.norm_inf();
                assert_eq!(res, 2.);
                println!("norm_inf({u}) = {res}");
            }
        }
    }

    #[test]
    fn other() {
        let u = Vector::from([-1., -2., -1.5, 1.8]);
        {
            let res = u.norm_1();
            assert_eq_float!(res, 6.3);
            println!("norm1({u}) = {res}");
        }
        {
            let res = u.norm();
            assert_eq_float!(res, 3.23883);
            println!("norm({u}) = {res}");
        }
        {
            let res = u.norm_inf();
            assert_eq!(res, 2.);
            println!("norm_inf({u}) = {res}");
        }
    }

    #[test]
    #[ignore = "rounding problem + approx eq feels strange"]
    #[allow(clippy::excessive_precision)]
    fn with_complex() {
        let u = Vector::from([cpl!(-5. - 7. i), cpl!(8. + 9. i), cpl!(0. - 2. i)]);
        {
            let res = u.norm_1();
            assert_eq!(
                res,
                cpl!(8.602_325_267_042_626_77 + 12.041_594_578_792 + 2., 0.)
            );
            println!("norm1({u}) = {res}");
        }
        {
            // * A complex number may have multiple sqrt
            // let res = u.norm();
            // assert_eq_float!(res, 3.23883);
            // println!("norm({}) = {}", u, res);
        }
        {
            // * Comparing two complex number has no sense
            // let res = u.norm_inf();
            // assert_eq!(res, 2.);
            // println!("norm_inf({}) = {}", u, res);
        }
    }
}
