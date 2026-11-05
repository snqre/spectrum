use super::*;

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + Eq,
	T::Scalar: Sign + NextWidest + FromPrimitive,
	T::Scalar: ConstZero,
	T::Scalar: Add<Output = T::Scalar>,
	T::Scalar: Sub<Output = T::Scalar>,
	T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar> {
	#[inline]
	#[must_use]
	pub fn lerp(self, rhs: Self, percentage: Self) -> Self {
		self + percentage * (rhs - self)
	}
}
