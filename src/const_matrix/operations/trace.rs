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
        // This is a square matrix
        for (i, line) in self.content.into_iter().enumerate() {
            // So if is a valid line number, then it's a valid column number
            init = f(
                init,
                line.into_iter().nth(i).unwrap_or_else(|| unreachable!()),
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
        // We know that there is AT LEAST one line and one column
        let mut acc = iter
            .next()
            .unwrap_or_else(|| unreachable!())
            .1
            .into_iter()
            .next()
            .unwrap_or_else(|| unreachable!());
        for (i, elt) in iter {
            // Since the matrix is square, a valid line number is a valid column number
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
