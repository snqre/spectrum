use super::*;
use conversion::AsFixedPointValue as _;

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + Eq + PartialOrd,
	T::Scalar: FromPrimitive,
	T::Scalar: Div<Output = T::Scalar>,
	T::Scalar: Rem<Output = T::Scalar>,
	T::Scalar: ConstOne + ConstZero {
	#[inline]
	#[must_use]
	pub fn round_up(self) -> Self {
		let n: T::Scalar = self.0;

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		let base: T::Scalar = T::Scalar::from_u128(T::BASE).unwrap();

		if n % base == T::Scalar::ZERO {
			let n: Self = Self(n);

			return n
		}

		if n >= T::Scalar::ZERO {
			let n: T::Scalar = n / base;
			let n: T::Scalar = n + T::Scalar::ONE;
			let n: T::Scalar = n * base;
			let n: Self = Self(n);

			return n
		}

		let n: T::Scalar = n / base;
		let n: T::Scalar = n * base;
		let n: Self = Self(n);

		n
	}
}

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + Eq + PartialOrd,
	T::Scalar: FromPrimitive,
	T::Scalar: Sub<Output = T::Scalar>,
	T::Scalar: Div<Output = T::Scalar>,
	T::Scalar: Rem<Output = T::Scalar>,
	T::Scalar: ConstOne + ConstZero {
	#[inline]
	#[must_use]
	pub fn round_down(self) -> Self {
		let n: T::Scalar = self.0;

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		let base: T::Scalar = T::Scalar::from_u128(T::BASE).unwrap();

		if n % base == T::Scalar::ZERO {
			let n: Self = Self(n);

			return n
		}

		if n >= T::Scalar::ZERO {
			let n: T::Scalar = n / base;
			let n: T::Scalar = n * base;
			let n: Self = Self(n);

			return n
		}

		let n: T::Scalar = n / base;
		let n: T::Scalar = n - T::Scalar::ONE;
		let n: T::Scalar = n * base;
		let n: Self = Self(n);

		n
	}
}

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + FromPrimitive,
	T::Scalar: Mul<Output = T::Scalar>,
	T::Scalar: Div<Output = T::Scalar> {
	#[inline]
	#[must_use]
	pub fn round_towards_zero(self) -> Self {
		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		let base: T::Scalar = T::Scalar::from_u128(T::BASE).unwrap();

		let n: T::Scalar = self.0;
		let n: T::Scalar = n / base;
		let n: T::Scalar = n * base;
		let n: Self = Self(n);

		n
	}
}

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + Eq + PartialOrd,
	T::Scalar: FromPrimitive,
	T::Scalar: Sub<Output = T::Scalar>,
	T::Scalar: Div<Output = T::Scalar>,
	T::Scalar: Rem<Output = T::Scalar>,
	T::Scalar: ConstOne + ConstZero {
	#[inline]
	#[must_use]
	pub fn round_away_from_zero(self) -> Self {
		let n: T::Scalar = self.0;

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		let base: T::Scalar = T::Scalar::from_u128(T::BASE).unwrap();

		if n % base == T::Scalar::ZERO {
			let n: Self = Self(n);

			return n
		}

		if n < T::Scalar::ZERO {
			let n: T::Scalar = n / base;
			let n: T::Scalar = n * base;
			let n: T::Scalar = n - base;
			let n: Self = Self(n);

			return n
		}

		let n: T::Scalar = n / base;
		let n: T::Scalar = n * base;
		let n: T::Scalar = n + base;
		let n: Self = Self(n);

		n
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[rstest::rstest]
	#[case(2_45.as_fixed_point_value(), 3_00.as_fixed_point_value())]
	#[case(2_00.as_fixed_point_value(), 2_00.as_fixed_point_value())]
	#[case((-2_45).as_fixed_point_value(), (-2_00).as_fixed_point_value())]
	#[case(0_50.as_fixed_point_value(), 1_00.as_fixed_point_value())]
	fn round_up(#[case] n: I64F2, #[case] expected: I64F2) {
		assert_eq!(n.round_up(), expected);
	}

	#[rstest::rstest]
	#[case(2_45.as_fixed_point_value(), 2_00.as_fixed_point_value())]
	fn round_down(#[case] n: I64F2, #[case] expected: I64F2) {
		assert_eq!(n.round_down(), expected);
	}

	#[rstest::rstest]
	#[case(2_45.as_fixed_point_value(), 2_00.as_fixed_point_value())]
	fn round_towards_zero(#[case] n: I64F2, #[case] expected: I64F2) {
		assert_eq!(n.round_towards_zero(), expected);
	}

	#[rstest::rstest]
	#[case(2_45.as_fixed_point_value(), 3_00.as_fixed_point_value())]
	fn round_away_from_zero(#[case] n: I64F2, #[case] expected: I64F2) {
		assert_eq!(n.round_away_from_zero(), expected);
	}
}
