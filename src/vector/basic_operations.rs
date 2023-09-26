mod add;
mod scalar_mul;
mod sub;

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
