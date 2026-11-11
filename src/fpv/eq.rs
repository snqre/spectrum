use super::*;

impl<T> PartialEq for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: PartialEq {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl<T> Eq for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Eq {}
