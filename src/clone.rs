use super::*;

impl<T> Clone for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Clone {
	#[inline]
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<T> Copy for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy {}
