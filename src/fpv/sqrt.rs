use super::*;
use arithmetic::{Sign, NextWidest};

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + Eq,
	T::Scalar: ConstOne + ConstZero,
	T::Scalar: Div<Output = T::Scalar>,
	T::Scalar: Sign + NextWidest + FromPrimitive,
	T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
 	<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar> {
	#[inline]
	#[must_use]
	pub fn sqrt(self) -> Self {
		let n: T::Scalar = self.0;

		if n == T::Scalar::ZERO || n == T::Scalar::ONE {
			let n: Self = Self(n);

			return n
		}

		let mut out: Self = Self(n / (T::Scalar::ONE + T::Scalar::ONE));
		let mut lst: Self;

		loop {
			lst = out;

			let n: Self = Self(n);

			out = out + (n / out);
			out = out / Self(T::Scalar::ONE + T::Scalar::ONE);

			if out == lst || out == (lst + Self(T::Scalar::ONE)) {
				break
			}
		}

		out
	}
}
