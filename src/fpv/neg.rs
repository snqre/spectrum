use super::*;

impl<T> Neg for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Neg<Output = T::Scalar> {
	type Output = Self;

	#[inline]
	fn neg(self) -> Self::Output {
		Self(-self.0)
	}
}
