use super::*;

impl<T> ConstZero for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: ConstZero {
	const ZERO: Self = Self(T::Scalar::ZERO);
}

impl<T> Zero for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Zero {
	fn is_zero(&self) -> bool {
    	self.0.is_zero()
	}

	fn zero() -> Self {
		Self(T::Scalar::zero())
	}
}
