use super::*;

impl<T> PartialOrd for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: PartialOrd {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		let lhs: &T::Scalar = &self.0;
		let rhs: &T::Scalar = &other.0;

		lhs.partial_cmp(rhs)
	}
}

impl<T> Ord for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Ord {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		let lhs: &T::Scalar = &self.0;
		let rhs: &T::Scalar = &other.0;

		lhs.cmp(rhs)
	}
}
