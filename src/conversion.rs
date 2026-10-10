// module will require better conversion

use super::*;

impl<T> FromPrimitive for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: FromPrimitive {
	fn from_u8(n: u8) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n.to_f64()? / T::F64_RECIPROCAL)?))
	}

	fn from_u16(n: u16) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n.to_f64()? / T::F64_RECIPROCAL)?))
	}

	fn from_u32(n: u32) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n.to_f64()? / T::F64_RECIPROCAL)?))
	}

	fn from_u64(n: u64) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n.to_f64()? / T::F64_RECIPROCAL)?))
	}

	fn from_u128(n: u128) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n.to_f64()? / T::F64_RECIPROCAL)?))
	}

	fn from_usize(n: usize) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n.to_f64()? / T::F64_RECIPROCAL)?))
	}

	fn from_i8(n: i8) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n.to_f64()? / T::F64_RECIPROCAL)?))
	}

	fn from_i16(n: i16) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n.to_f64()? / T::F64_RECIPROCAL)?))
	}

	fn from_i32(n: i32) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n.to_f64()? / T::F64_RECIPROCAL)?))
	}

	fn from_i64(n: i64) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n.to_f64()? / T::F64_RECIPROCAL)?))
	}

	fn from_i128(n: i128) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n.to_f64()? / T::F64_RECIPROCAL)?))
	}

	fn from_isize(n: isize) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n.to_f64()? / T::F64_RECIPROCAL)?))
	}

	fn from_f32(n: f32) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n.to_f64()? / T::F64_RECIPROCAL)?))
	}

	fn from_f64(n: f64) -> Option<Self> {
		Some(Self(T::Scalar::from_f64(n / T::F64_RECIPROCAL)?))
	}
}

impl<T> ToPrimitive for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + ToPrimitive {
	fn to_u8(&self) -> Option<u8> {
		self.to_f64()?.trunc().to_u8()
	}

	fn to_u16(&self) -> Option<u16> {
		self.to_f64()?.trunc().to_u16()
	}

	fn to_u32(&self) -> Option<u32> {
		self.to_f64()?.trunc().to_u32()
	}

	fn to_u64(&self) -> Option<u64> {
		self.to_f64()?.trunc().to_u64()
	}

	fn to_u128(&self) -> Option<u128> {
		self.to_f64()?.trunc().to_u128()
	}

	fn to_usize(&self) -> Option<usize> {
		self.to_f64()?.trunc().to_usize()
	}

	fn to_i8(&self) -> Option<i8> {
		self.to_f64()?.trunc().to_i8()
	}

	fn to_i16(&self) -> Option<i16> {
		self.to_f64()?.trunc().to_i16()
	}

	fn to_i32(&self) -> Option<i32> {
		self.to_f64()?.trunc().to_i32()
	}

	fn to_i64(&self) -> Option<i64> {
		self.to_f64()?.trunc().to_i64()
	}

	fn to_i128(&self) -> Option<i128> {
		self.to_f64()?.trunc().to_i128()
	}

	fn to_isize(&self) -> Option<isize> {
		self.to_f64()?.trunc().to_isize()
	}

	fn to_f32(&self) -> Option<f32> {
		Some(self.0.to_f32()? * T::F32_RECIPROCAL)
	}

	fn to_f64(&self) -> Option<f64> {
		Some(self.0.to_f64()? * T::F64_RECIPROCAL)
	}
}

#[allow(clippy::unwrap_used)]
#[allow(clippy::float_cmp)]
#[cfg(test)]
mod test {
	use super::*;

	#[rstest::rstest]
	#[case(20_00, 20)]
	#[case(15_00, 15)]
	#[case(50, 0)]
	fn to_u8(#[case] n: i64, #[case] answer: u8) {
		assert_eq!(FixedPointValue::<With<2, i64>>(n).to_u8().unwrap(), answer);
	}

	#[rstest::rstest]
	#[case(20_00, 20)]
	#[case(15_00, 15)]
	#[case(50, 0)]
	fn to_u16(#[case] n: i64, #[case] answer: u16) {
		assert_eq!(FixedPointValue::<With<2, i64>>(n).to_u16().unwrap(), answer);
	}

	#[rstest::rstest]
	#[case(20_00, 20.0)]
	#[case(15_00, 15.0)]
	#[case(50, 0.5)]
	fn to_f32(#[case] n: i64, #[case] answer: f32) {
		assert_eq!(FixedPointValue::<With<2, i64>>(n).to_f32().unwrap(), answer);
	}

	#[rstest::rstest]
	#[case(20_00, 20.0)]
	#[case(15_00, 15.0)]
	#[case(50, 0.5)]
	fn to_f64(#[case] n: i64, #[case] answer: f64) {
		assert_eq!(FixedPointValue::<With<2, i64>>(n).to_f64().unwrap(), answer);
	}
}
