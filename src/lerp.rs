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

#[cfg(test)]
mod test {
	use super::*;

	#[rstest::rstest]
	#[case(2_00, 3_00, 0_25, 2_25)]
	#[case(0_00, 10_00, 0_50, 5_00)]
	#[case(2_00, 4_00, 0_00, 2_00)]
	#[case(2_00, 4_00, 1_00, 4_00)]
	#[case(10_00, 20_00, 0_75, 17_50)]
	#[case(-10_00, 10_00, 0_50, 0_00)]
	fn lerp(#[case] x: i64, #[case] y: i64, #[case] percentage: i64, #[case] expected: i64) {
		let x: I64F2 = x.as_fixed_point_value();
		let y: I64F2 = y.as_fixed_point_value();
		let percentage: I64F2 = percentage.as_fixed_point_value();
		let expected: I64F2 = expected.as_fixed_point_value();
		let got: I64F2 = x.lerp(y, percentage);

		assert_eq!(got, expected);
	}
}
