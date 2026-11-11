use super::*;
use arithmetic::{Sign, NextWidest};
use conversion::AsFixedPointValue as _;

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + Eq + PartialOrd,
	T::Scalar: Sign + NextWidest + FromPrimitive + ToPrimitive,
	T::Scalar: ConstZero,
	T::Scalar: ConstOne,
	T::Scalar: Add<Output = T::Scalar>,
	T::Scalar: Sub<Output = T::Scalar>,
	T::Scalar: Rem<Output = T::Scalar>,
	T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Copy + PartialOrd,
	<T::Scalar as NextWidest>::Scalar: ToPrimitive + FromPrimitive,
	<T::Scalar as NextWidest>::Scalar: ConstZero,
	<T::Scalar as NextWidest>::Scalar: ConstOne,
	<T::Scalar as NextWidest>::Scalar: Add<Output = <T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Sub<Output = <T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Rem<Output = <T::Scalar as NextWidest>::Scalar> {

	#[inline]
	pub fn sin(self) -> Self {
		let (out, _) = self.sincos();

		out
	}

	#[inline]
	pub fn cos(self) -> Self {
		let (_, out) = self.sincos();

		out
	}

	#[inline]
	pub fn tan(self) -> Self {
		let (sin, cos) = self.sincos();

		sin / cos
	}

	fn sincos(self) -> (Self, Self) {
		let n: T::Scalar = self.0;
		let n: <T::Scalar as NextWidest>::Scalar = n.into_next_widest_scalar();
		let n_0: <T::Scalar as NextWidest>::Scalar = <T::Scalar as NextWidest>::Scalar::zero();
		let n_2: <T::Scalar as NextWidest>::Scalar = <T::Scalar as NextWidest>::Scalar::one() + <T::Scalar as NextWidest>::Scalar::one();
		let n_4: <T::Scalar as NextWidest>::Scalar = <T::Scalar as NextWidest>::Scalar::one() + <T::Scalar as NextWidest>::Scalar::one() + <T::Scalar as NextWidest>::Scalar::one() + <T::Scalar as NextWidest>::Scalar::one();

		let pi: u128 = T::PI;

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		let pi: <T::Scalar as NextWidest>::Scalar = <T::Scalar as NextWidest>::Scalar::from_u128(pi).unwrap();

		let tau: <T::Scalar as NextWidest>::Scalar = pi * n_2;

		let half_pi: <T::Scalar as NextWidest>::Scalar = pi / n_2;

		let angle: <T::Scalar as NextWidest>::Scalar = Self::trigonometric_angle_normalization(n, n_0, tau);

		let (quadrant, remainder) = Self::trigonometric_quadrant_extraction(angle, half_pi, n_4);

		let lut_len: usize = T::SIN_LUT.len();

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		let lut_len_scalar: T::Scalar = T::Scalar::from_usize(lut_len).unwrap();
		let lut_len_scalar: <T::Scalar as NextWidest>::Scalar = lut_len_scalar.into_next_widest_scalar();

		let position: <T::Scalar as NextWidest>::Scalar = Self::trigonometric_position(remainder, half_pi, lut_len_scalar);

		let (lo, hi, fr) = Self::trigonometric_keys(position, lut_len);

		let (sin, cos) = Self::trigonometric_lut_interpolation(lo, hi, fr);
		let (sin, cos) = Self::trigonometric_quadrant_application(quadrant, sin, cos);

		(Self(sin), Self(cos))
	}

	fn trigonometric_angle_normalization(n: <T::Scalar as NextWidest>::Scalar, n_0: <T::Scalar as NextWidest>::Scalar, tau: <T::Scalar as NextWidest>::Scalar) -> <T::Scalar as NextWidest>::Scalar {
		let mut angle: <T::Scalar as NextWidest>::Scalar = n % tau;

		if angle < n_0 {
			angle = angle + tau;
		}

		angle
	}

	fn trigonometric_quadrant_extraction(angle: <T::Scalar as NextWidest>::Scalar, half_pi: <T::Scalar as NextWidest>::Scalar, n_4: <T::Scalar as NextWidest>::Scalar) -> (usize, <T::Scalar as NextWidest>::Scalar) {
		let quadrant: <T::Scalar as NextWidest>::Scalar = (angle / half_pi) % n_4;
		let quadrant: usize = match quadrant.to_u64() {
			#[allow(clippy::cast_possible_truncation)]
			Some(x) => x as usize & 3,
			None => 0
		};

		let remainder: <T::Scalar as NextWidest>::Scalar = angle % half_pi;

		(quadrant, remainder)
	}

	fn trigonometric_quadrant_application(quadrant: usize, sin_r: T::Scalar, cos_r: T::Scalar) -> (T::Scalar, T::Scalar)
	where
		T: Denomination,
		T::Scalar: Copy + ConstZero + Sub<Output = T::Scalar> {
		let zero: T::Scalar = T::Scalar::zero();

		match quadrant {
			0 => (sin_r, cos_r),
			1 => (cos_r, zero - sin_r),
			2 => (zero - sin_r, zero - cos_r),
			_ => (zero - cos_r, sin_r)
		}
	}

	fn trigonometric_position(remainder: <T::Scalar as NextWidest>::Scalar, half_pi: <T::Scalar as NextWidest>::Scalar, lut_len: <T::Scalar as NextWidest>::Scalar) -> <T::Scalar as NextWidest>::Scalar {
		(remainder * lut_len) / half_pi
	}

	/// # Overview
	///
	/// Converts the continuous position into discrete array indices and a fractional remainder required for linear interpolation between adjacent LUT table buckets.
	///
	/// ## Result
	/// - `lo`: The lower bounding table key, clamped to `lut_len - 1`.
	/// - `hi`: The upper bounding table key (`lo + 1`), clamped to `lut_len - 1`.
	/// - `fr`: The fractional interpolation distance (`position - lo`) in scalar format.
	///
	/// ## Params
	/// - `position`: Continuous floating/scaled position within the LUT.
	/// - `lut_len`: Maximum capacity dimension of the LUT.
	fn trigonometric_keys(position: <T::Scalar as NextWidest>::Scalar, lut_len: usize) -> (usize, usize, <T::Scalar as NextWidest>::Scalar) {
		let position_u64: u64 = position.to_u64().unwrap_or_default();

		#[allow(clippy::cast_possible_truncation)]
		let lo = (position_u64 as usize).min(lut_len - 1);
		let hi = (lo + 1).min(lut_len - 1);

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		let lo_scalar = <T::Scalar as NextWidest>::Scalar::from_u64(lo as u64).unwrap();
		let fr = position - lo_scalar;

		(lo, hi, fr)
	}

	fn trigonometric_lut_interpolation(lo: usize, hi: usize, fr: <T::Scalar as NextWidest>::Scalar) -> (T::Scalar, T::Scalar) {
		let zero: <T::Scalar as NextWidest>::Scalar = <T::Scalar as NextWidest>::Scalar::zero();

		let sin_lo: <T::Scalar as NextWidest>::Scalar = <T::Scalar as NextWidest>::Scalar::from_u128(T::SIN_LUT[lo]).unwrap_or(zero);
		let sin_hi: <T::Scalar as NextWidest>::Scalar = <T::Scalar as NextWidest>::Scalar::from_u128(T::SIN_LUT[hi]).unwrap_or(zero);
		let cos_lo: <T::Scalar as NextWidest>::Scalar = <T::Scalar as NextWidest>::Scalar::from_u128(T::COS_LUT[lo]).unwrap_or(zero);
		let cos_hi: <T::Scalar as NextWidest>::Scalar = <T::Scalar as NextWidest>::Scalar::from_u128(T::SIN_LUT[hi]).unwrap_or(zero);

		let sin_lut_len: usize = T::SIN_LUT.len();

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		let sin_lut_len: <T::Scalar as NextWidest>::Scalar = <T::Scalar as NextWidest>::Scalar::from_usize(sin_lut_len).unwrap();

		let cos_lut_len: usize = T::COS_LUT.len();

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		let cos_lut_len: <T::Scalar as NextWidest>::Scalar = <T::Scalar as NextWidest>::Scalar::from_usize(cos_lut_len).unwrap();

		let two: <T::Scalar as NextWidest>::Scalar = <T::Scalar as NextWidest>::Scalar::one() + <T::Scalar as NextWidest>::Scalar::one();

		let sin_half_lut: <T::Scalar as NextWidest>::Scalar = sin_lut_len / two;
		let cos_half_lut: <T::Scalar as NextWidest>::Scalar = cos_lut_len / two;

		let sin_r: <T::Scalar as NextWidest>::Scalar = sin_lo + ((sin_hi - sin_lo) * fr + sin_half_lut) / sin_lut_len;
		let cos_r: <T::Scalar as NextWidest>::Scalar = cos_lo + ((cos_hi - cos_lo) * fr + cos_half_lut) / cos_lut_len;

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		let sin_r: T::Scalar = sin_r.try_into().ok().unwrap();

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		let cos_r: T::Scalar = cos_r.try_into().ok().unwrap();

		(sin_r, cos_r)
	}
}

#[allow(clippy::mistyped_literal_suffixes)]
#[allow(clippy::large_digit_groups)]
#[cfg(test)]
mod test {
	use super::*;

	#[rstest::rstest]
	#[case(0, 0, 1_00)]
	#[case(-52, -49, 86)]
	#[case(-1_05, -86, 49)]
	fn sincos_at_exponent_2(#[case] value: i64, #[case] expected_sin: i64, #[case] expected_cos: i64) {
		let value: I64F2 = value.as_fixed_point_value();
		let expected_sin: I64F2 = expected_sin.as_fixed_point_value();
		let expected_cos: I64F2 = expected_cos.as_fixed_point_value();

		let (sin, cos) = value.sincos();

		assert_eq!(sin, expected_sin, "failed sin");
		assert_eq!(cos, expected_cos, "failed cos");
	}

	#[rstest::rstest]
	#[case(0, 0, 1_000000000)]
	#[case(-0_520000000, -0_496897049, 0_867809496)]
	#[case(-1_050000000, -0_867809496, 0_496897049)]
	fn sincos_at_exponent_9(#[case] value: i64, #[case] expected_sin: i64, #[case] expected_cos: i64) {
		let value: I64F9 = value.as_fixed_point_value();
		let expected_sin: I64F9 = expected_sin.as_fixed_point_value();
		let expected_cos: I64F9 = expected_cos.as_fixed_point_value();

		let (sin, cos) = value.sincos();

		assert_eq!(sin, expected_sin, "failed sin");
		assert_eq!(cos, expected_cos, "failed cos");
	}

	#[rstest::rstest]
	#[case(0_00.as_fixed_point_value(), 0_00.as_fixed_point_value())]
	#[case((-0_52).as_fixed_point_value(), (-0_56).as_fixed_point_value())]
	#[case(1_00.as_fixed_point_value(), 1_55.as_fixed_point_value())]
	#[case((-1_00).as_fixed_point_value(), (-1_58).as_fixed_point_value())]
	#[case(0_50.as_fixed_point_value(), 0_54.as_fixed_point_value())]
	#[case((-0_50).as_fixed_point_value(), (-0_55).as_fixed_point_value())]
	#[case(0_25.as_fixed_point_value(), 0_25.as_fixed_point_value())]
	#[case((-0_25).as_fixed_point_value(), (-0_25).as_fixed_point_value())]
	fn tan(#[case] n: I64F2, #[case] expected: I64F2) {
		let got: I64F2 = n.tan();

		assert_eq!(got, expected);
	}
}
