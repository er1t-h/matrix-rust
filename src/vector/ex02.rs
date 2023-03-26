use std::ops::{MulAssign, AddAssign};

use crate::{Vector, traits::{Space, Float}};

pub fn lerp<V, Sliding>(u: &V, v: &V, t: &Sliding) -> V
where
	Sliding: Into<f64>,
	V: Clone,
	for<'a> V: AddAssign<&'a V> + MulAssign<&'a Sliding>
{
	let bounds: f64 = (*t).into();
}

pub fn lerp<V, Sliding>(u: &V, v: &V, t: &Sliding) -> V
where
	Sliding: Into<f64>,
	V: Clone,
	for<'a> V: AddAssign<&'a V> + MulAssign<&'a Sliding>
{
	let mut accumulator = u.clone();
	accumulator += v;
	accumulator *= t;
	accumulator
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{vector::ex02::lerp, Vector, Matrix};

	#[test]
	fn example() {
		{
			let res = lerp(&0., &1., &0.);
			assert_eq!(res, 0.);
			println!("{}", res);
		}
		{
			let res = lerp(&0., &1., &1.);
			assert_eq!(res, 1.);
			println!("{}", res);
		}
		{
			let res = lerp(&0., &1., &0.5);
			assert_eq!(res, 0.5);
			println!("{}", res);
		}
		{
			let res = lerp(&21., &42., &0.3);
			assert_eq!(res, 27.3);
			println!("{}", res);
		}
		{
			let res = lerp(&Vector::from([2., 1.]), &Vector::from([4., 2.]), &0.3);
			assert_eq!(res, [2.6, 1.3]);
			println!("{}", res);
		}
		{
			let res = lerp(&Matrix::from([[2., 1.], [3., 4.]]), &Matrix::from([[20., 10.], [30., 40.]]), &0.5);
			assert_eq!(res, [[11., 5.5], [16.5, 22.]]);
			println!("{}", res);
		}
	}

	#[test]
	fn errors() {
		assert_eq!(res)
	}
}
