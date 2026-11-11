use super::*;

impl<const A: Dimension, B> Vec<A, B>
where
	B: Space<A>,
	B::Scalar: Copy + Add<Output = B::Scalar> + Sub<Output = B::Scalar> + Mul<Output = B::Scalar> {
	pub fn lerp(self, rhs: &Self, percentage: B::Scalar) -> Self {
		Self(core::array::from_fn(|key| {
			let x: B::Scalar = self.0[key];
			let y: B::Scalar = rhs.0[key];

			x + (y - x) * percentage
		}))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use fpv::conversion::AsFixedPointValue as _;

	#[rstest::rstest]
	#[case(Vec2DI16F2::new(0_00i16.as_fixed_point_value(), 0_00.as_fixed_point_value()), Vec2DI16F2::new(10_00.as_fixed_point_value(), 10_00.as_fixed_point_value()), 0_50.as_fixed_point_value(), Vec2DI16F2::new(5_00.as_fixed_point_value(), 5_00.as_fixed_point_value()))]
	fn lerp(#[case] lhs: Vec2DI16F2, #[case] rhs: Vec2DI16F2, #[case] percentage: fpv::I16F2, #[case] expected: Vec2DI16F2) {
		let got: Vec<_, _> = lhs.lerp(&rhs, percentage);

		assert_eq!(got, expected);
	}
}
