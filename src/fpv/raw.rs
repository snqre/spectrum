use super::*;
use conversion::AsFixedPointValue as _;

pub trait Mul<T>
where
	T: Denomination {
	#[must_use]
	fn raw_mul(self, rhs: T::Scalar) -> Self;
}

pub trait Div<T>
where
	T: Denomination {
	#[must_use]
	fn raw_div(self, rhs: T::Scalar) -> Self;
}

impl<T> Mul<T> for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: core::ops::Mul<Output = T::Scalar> {
	fn raw_mul(self, rhs: T::Scalar) -> Self {
    	Self(self.into_inner() * rhs)
	}
}

impl<T> Div<T> for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: core::ops::Div<Output = T::Scalar> {
	fn raw_div(self, rhs: T::Scalar) -> Self {
    	Self(self.into_inner() / rhs)
	}
}
