use crate::const_matrix::ConstMatrix;
use std::ops::Add;

impl<K, const SIZE: usize> ConstMatrix<K, SIZE, SIZE> {
    ///
    /// Applies the function `f` on each number of the diagonal of the matrix.
    /// The first accumulator will be `init`.
    ///
    /// # Panics
    /// Never.
    ///
    pub fn trace_init_fn<F: Fn(K, K) -> K>(self, mut init: K, f: F) -> K {
        // `self` is a square matrix, and i is an index from `self`
        for (i, elt) in self.content.into_iter().enumerate() {
            init = f(
                init,
                elt.into_iter().nth(i).unwrap_or_else(|| unreachable!()),
            );
        }
        init
    }
}

impl<K, const SIZE: usize> ConstMatrix<K, SIZE, SIZE> {
    ///
    /// Applies the function `f` on each number of the diagonal of the matrix.
    /// The first accumulator will be the top-left element of the matrix.
    ///
    /// The trace function can be implemented as follow:
    /// ```
    /// use matrix::const_matrix::ConstMatrix;
    /// let u = ConstMatrix::from([
    ///     [3., 2.],
    ///     [1., 4.]
    /// ]);
    /// let trace = u.trace_fn(|accumulator, next| accumulator + next);
    /// assert_eq!(trace, 7.)
    /// ```
    ///
    /// # Panics
    /// Never.
    ///
    pub fn trace_fn<F: Fn(K, K) -> K>(self, f: F) -> K {
        let mut iter = self.content.into_iter().enumerate();
        // `self` is at least a 1x1 matrix, and this lines takes the first element of the first line
        let mut acc = iter
            .next()
            .unwrap_or_else(|| unreachable!())
            .1
            .into_iter()
            .next()
            .unwrap_or_else(|| unreachable!());
        // `self` is a square matrix, so `i` will remain within bounds
        for (i, elt) in iter {
            acc = f(
                acc,
                elt.into_iter().nth(i).unwrap_or_else(|| unreachable!()),
            );
        }
        acc
    }
}

impl<K, const SIZE: usize> ConstMatrix<K, SIZE, SIZE>
where
    K: Add<Output = K>,
{
    ///
    /// Computes the trace of the matrix.
    ///
    pub fn trace(self) -> K {
        self.trace_fn(|acc, elt| acc + elt)
    }
}

#[cfg(test)]
mod test {
    use crate::const_matrix::ConstMatrix;

    #[test]
    fn example() {
        {
            let u = ConstMatrix::from([[1., 0.], [0., 1.]]);
            let res = u.trace();
            assert_eq!(res, 2.0);
        }
        {
            let u = ConstMatrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
            let res = u.trace();
            assert_eq!(res, 9.0);
        }
        {
            let u = ConstMatrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
            let res = u.trace();
            assert_eq!(res, -21.0);
        }
    }
}
