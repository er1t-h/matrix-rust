use std::ops::{Add, Mul, Sub};

use crate::Matrix;

// Matrix:
// 0 1
// 2 3
#[inline(always)]
fn determinant_2<K>(content: &[&K; 4]) -> K
where
    K: Clone + Default,
    for<'a> &'a K: Mul<&'a K, Output = K> + Sub<&'a K, Output = K>,
{
    &(content[0] * content[3]) - &(content[1] * content[2])
}

// Matrix:
// 0 1 2
// 3 4 5
// 6 7 8
#[inline(always)]
fn determinant_3<K>(content: &[&K; 9]) -> K
where
    K: Clone + Default,
    for<'a> &'a K: Mul<&'a K, Output = K> + Sub<&'a K, Output = K> + Add<&'a K, Output = K>,
{
    &(&(content[0] * &determinant_2(&[content[4], content[5], content[7], content[8]]))
        - &(content[1] * &determinant_2(&[content[3], content[5], content[6], content[8]])))
        + &(content[2] * &determinant_2(&[content[3], content[4], content[6], content[7]]))
}

// Matrix:
//  0  1  2  3
//  4  5  6  7
//  8  9 10 11
// 12 13 14 15
#[inline(always)]
fn determinant_4<K>(content: &[&K; 16]) -> K
where
    K: Clone + Default,
    for<'a> &'a K: Mul<&'a K, Output = K> + Sub<&'a K, Output = K> + Add<&'a K, Output = K>,
{
    &(&(&(content[0]
        * &determinant_3(&[
            content[5],
            content[6],
            content[7],
            content[9],
            content[10],
            content[11],
            content[13],
            content[14],
            content[15],
        ]))
        - &(content[1]
            * &determinant_3(&[
                content[4],
                content[6],
                content[7],
                content[8],
                content[10],
                content[11],
                content[12],
                content[14],
                content[15],
            ])))
        + &(content[2]
            * &determinant_3(&[
                content[4],
                content[5],
                content[7],
                content[8],
                content[9],
                content[11],
                content[12],
                content[13],
                content[15],
            ])))
        - &(content[3]
            * &determinant_3(&[
                content[4],
                content[5],
                content[6],
                content[8],
                content[9],
                content[10],
                content[12],
                content[13],
                content[14],
            ]))
}

impl<'a, K> Matrix<K>
where
    K: Clone + Default + 'a,
    for<'b> &'b K: Sub<&'b K, Output = K> + Mul<&'b K, Output = K> + Add<&'b K, Output = K>,
{
    // ! I know it looks terrible, but it's the only way to do it without cloning the matrix
    // ! Moreover, it's
    pub fn determinant(&self) -> Result<K, ()> {
        match self.size() {
            (1, 1) => Ok(self.content[0].clone()),
            (2, 2) => Ok(determinant_2(&[&self[0], &self[1], &self[2], &self[3]])),
            (3, 3) => Ok(determinant_3(&[
                &self[0], &self[1], &self[2], &self[3], &self[4], &self[5], &self[6], &self[7],
                &self[8],
            ])),
            (4, 4) => Ok(determinant_4(&[
                &self[0], &self[1], &self[2], &self[3], &self[4], &self[5], &self[6], &self[7],
                &self[8], &self[9], &self[10], &self[11], &self[12], &self[13], &self[14],
                &self[15],
            ])),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Matrix;

    #[test]
    fn example() {
        {
            let u = Matrix::from([[1., -1.], [-1., 1.]]);
            let res = u.determinant().unwrap();
            assert_eq!(res, 0.);
            println!("det({}) = {:?}", u, res);
            // 0.0
        }
        {
            let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
            let res = u.determinant().unwrap();
            assert_eq!(res, 8.);
            println!("det({}) = {:?}", u, res);
            // 8.0
        }
        {
            let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
            let res = u.determinant().unwrap();
            assert_eq!(res, -174.);
            println!("det({}) = {:?}", u, res);
            // -174.0
        }
        {
            let u = Matrix::from([
                [8., 5., -2., 4.],
                [4., 2.5, 20., 4.],
                [8., 5., 1., 4.],
                [28., -4., 17., 1.],
            ]);
            let res = u.determinant().unwrap();
            assert_eq!(res, 1032.);
            println!("det({}) = {:?}", u, res);
            // 1032
        }
    }
}
