use super::*;

impl<T> Bounded for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Bounded {
	#[inline]
	fn max_value() -> Self {
		Self(T::Scalar::max_value())
	}

	#[inline]
	fn min_value() -> Self {
		Self(T::Scalar::min_value())
	}
}
