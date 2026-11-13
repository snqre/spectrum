use super::*;
use arithmetic::Sign;

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Zero + Eq,
	T::Scalar: Sub<Output = T::Scalar> {

	/// Converts the fixed-point value to its negative counterpart.
	///
	/// > If the value is zero, it is returned unchanged.
	///
	/// # Example
	///
	/// ```rust
	/// use reliq::fpv::I64F2;
	/// use reliq::fpv::conversion::AsFixedPointValue as _;
	///
	/// let value: I64F2 = 5_00.as_fixed_point_value();
	/// let value: I64F2 = value.to_negative();
	///
	/// let expected: I64F2 = (-5_00).as_fixed_point_value();
	///
	/// assert_eq!(value, expected);
	/// ```
	#[inline]
	pub fn to_negative(self) -> Self {
		let n: T::Scalar = self.0;

		if n == T::Scalar::zero() {
			return Self(n)
		}

		let n: T::Scalar = T::Scalar::zero() - n;
		let n: Self = Self(n);

		n
	}
}

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Zero + Ord,
	T::Scalar: Sub<Output = T::Scalar> {
	#[inline]
	pub fn to_positive(self) -> Self {
		let n: T::Scalar = self.0;

		if n >= T::Scalar::zero() {
			return Self(n)
		}

		let n: T::Scalar = T::Scalar::zero() - n;
		let n: Self = Self(n);

		n
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use conversion::AsFixedPointValue as _;

	#[rstest::rstest]
	#[case(5_00.as_fixed_point_value(), -5_00.as_fixed_point_value())]
	fn to_negative(#[case] value: I64F2, #[case] expected: I64F2) {
		let got: I64F2 = value.to_negative();

		assert_eq!(got, expected);
	}

	#[rstest::rstest]
	#[case(-5_00.as_fixed_point_value(), 5_00.as_fixed_point_value())]
	fn to_positive(#[case] value: I64F2, #[case] expected: I64F2) {
		let got: I64F2 = value.to_positive();

		assert_eq!(got, expected);
	}
}
