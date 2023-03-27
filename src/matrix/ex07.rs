use std::{iter::Sum, ops::Mul};

use crate::{Matrix, Vector};

impl<K> Matrix<K>
where
    K: Clone + Sum,
    for<'a> &'a K: Mul<&'a K, Output = K>,
{
    pub fn safe_mul_vec(&self, vec: &Vector<K>) -> Result<Vector<K>, ()> {
        if self.dimensions.width != vec.len() {
            return Err(());
        }
        let mut result_vec: Vector<K> = Vector::new();
        for (index, elt) in vec.iter().enumerate() {
            result_vec.append(self.get_column(index).unwrap().map(|x| x * elt).sum())
        }
        Ok(result_vec)
    }
}

#[cfg(test)]
mod test {
    use crate::{Matrix, Vector};

    #[test]
    fn example() {
        {
            let u = Matrix::from([[1., 0.], [0., 1.]]);
            let v = Vector::from([4., 2.]);
            let res = u.safe_mul_vec(&v).unwrap();
            assert_eq!(res, [4., 2.]);
            println!("{} * {} = {}", u, v, res);
            // [4.]
            // [2.]
        }
        {
            let u = Matrix::from([[2., 0.], [0., 2.]]);
            let v = Vector::from([4., 2.]);
            let res = u.safe_mul_vec(&v).unwrap();
            assert_eq!(res, [8., 4.]);
            println!("{} * {} = {}", u, v, res);
            // [8.]
            // [4.]
        }
        {
            let u = Matrix::from([[2., -2.], [-2., 2.]]);
            let v = Vector::from([4., 2.]);
            let res = u.safe_mul_vec(&v).unwrap();
            assert_eq!(res, [4., -4.]);
            println!("{} * {} = {}", u, v, res);
            // [4.]
            // [-4.]
        }
    }
}
