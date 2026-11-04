use super::*;
use ethnum::u256;
use ethnum::i256;

pub trait Sign {
	const SIGNED: bool;
}

impl Sign for u8 {
	const SIGNED: bool = false;
}

impl Sign for u16 {
	const SIGNED: bool = false;
}

impl Sign for u32 {
	const SIGNED: bool = false;
}

impl Sign for u64 {
	const SIGNED: bool = false;
}

impl Sign for u128 {
	const SIGNED: bool = false;
}

impl Sign for usize {
	const SIGNED: bool = false;
}

impl Sign for i8 {
	const SIGNED: bool = true;
}

impl Sign for i16 {
	const SIGNED: bool = true;
}

impl Sign for i32 {
	const SIGNED: bool = true;
}

impl Sign for i64 {
	const SIGNED: bool = true;
}

impl Sign for i128 {
	const SIGNED: bool = true;
}

impl Sign for isize {
	const SIGNED: bool = true;
}


pub trait NextWidest {
	type Scalar;

	fn into_next_widest_scalar(self) -> Self::Scalar;
}

impl NextWidest for u8 {
	type Scalar = u64;

	#[inline]
	fn into_next_widest_scalar(self) -> Self::Scalar {
		self.as_()
	}
}

impl NextWidest for u16 {
	type Scalar = u64;

	#[inline]
	fn into_next_widest_scalar(self) -> Self::Scalar {
		self.as_()
	}
}

impl NextWidest for u32 {
	type Scalar = u64;

	#[inline]
	fn into_next_widest_scalar(self) -> Self::Scalar {
    	self.as_()
	}
}

impl NextWidest for u64 {
	type Scalar = u128;

	#[inline]
	fn into_next_widest_scalar(self) -> Self::Scalar {
    	self.as_()
	}
}

impl NextWidest for u128 {
	type Scalar = u256;

	#[inline]
	fn into_next_widest_scalar(self) -> Self::Scalar {
		self.into()
	}
}

#[cfg(target_pointer_width = "64")]
impl NextWidest for usize {
    type Scalar = u128;

    #[inline]
    fn into_next_widest_scalar(self) -> Self::Scalar {
        self.as_()
    }
}

impl NextWidest for i8 {
	type Scalar = i64;

	#[inline]
	fn into_next_widest_scalar(self) -> Self::Scalar {
    	self.as_()
	}
}

impl NextWidest for i16 {
	type Scalar = i64;

	#[inline]
	fn into_next_widest_scalar(self) -> Self::Scalar {
    	self.as_()
	}
}

impl NextWidest for i32 {
	type Scalar = i64;

	#[inline]
	fn into_next_widest_scalar(self) -> Self::Scalar {
    	self.as_()
	}
}

impl NextWidest for i64 {
	type Scalar = i128;

	#[inline]
	fn into_next_widest_scalar(self) -> Self::Scalar {
    	self.as_()
	}
}

impl NextWidest for i128 {
	type Scalar = i256;

	#[inline]
	fn into_next_widest_scalar(self) -> Self::Scalar {
    	self.into()
	}
}

#[cfg(target_pointer_width = "64")]
impl NextWidest for isize {
    type Scalar = i128;

    #[inline]
    fn into_next_widest_scalar(self) -> Self::Scalar {
        self.as_()
    }
}

impl<T> FixedPointValue<T>
where
    T: Denomination,
    T::Scalar: Eq + ConstZero + NextWidest,
	T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar> {

	#[inline]
    pub(crate) fn mul(lhs: T::Scalar, rhs: T::Scalar) -> Result<T::Scalar>
    where
        T::Scalar: Sign + FromPrimitive {
        debug_assert!(matches!(size_of::<T::Scalar>(), 1..=16));

        if matches!(T::EXPONENT, 0 | 38..) {
            return Err(Error::UnsupportedExponent)
        }

        if T::Scalar::SIGNED {
			let scale: T::Scalar = unsafe {
				let scale: i128 = T::BASE.to_i128().unwrap_unchecked();
				let scale: T::Scalar = T::Scalar::from_i128(scale).unwrap_unchecked();

				scale
			};

			Self::muldiv(lhs, rhs, scale)
        } else {
			let scale: T::Scalar = unsafe {

				T::Scalar::from_u128(T::BASE).unwrap_unchecked()
			};

			Self::muldiv(lhs, rhs, scale)
        }
    }

    #[inline]
    pub(crate) fn div(lhs: T::Scalar, rhs: T::Scalar) -> Result<T::Scalar>
    where
        T::Scalar: Sign + FromPrimitive {
        debug_assert!(matches!(size_of::<T::Scalar>(), 1..=16));

        if matches!(T::EXPONENT, 0 | 38..) {
            return Err(Error::UnsupportedExponent)
        }

		if T::Scalar::SIGNED {
			let scale: T::Scalar = unsafe {
				let scale: i128 = T::BASE.to_i128().unwrap_unchecked();
				let scale: T::Scalar = T::Scalar::from_i128(scale).unwrap_unchecked();

				scale
			};

			Self::muldiv(lhs, scale, rhs)
		} else {
			let scale: T::Scalar = unsafe {
				T::Scalar::from_u128(T::BASE).unwrap_unchecked()
			};

			Self::muldiv(lhs, scale, rhs)
		}
    }

	#[inline]
	pub(crate) fn muldiv(lhs: T::Scalar, rhs: T::Scalar, denominator: T::Scalar) -> Result<T::Scalar> {
        debug_assert!(matches!(size_of::<T::Scalar>(), 1..=16));

		#[cold]
		if denominator == T::Scalar::ZERO {
			return Err(Error::DivisionByZero)
		}

		let lhs: <T::Scalar as NextWidest>::Scalar = lhs.into_next_widest_scalar();
		let rhs: <T::Scalar as NextWidest>::Scalar = rhs.into_next_widest_scalar();
		let denominator: <T::Scalar as NextWidest>::Scalar = denominator.into_next_widest_scalar();

		let out: <T::Scalar as NextWidest>::Scalar = (lhs * rhs) / denominator;

		let Ok(out) = out.try_into() else {

			#[cold]
			return Err(Error::ArithmeticOverflow)
		};

		Ok(out)
	}
}

impl<T> Add for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Add<T::Scalar, Output = T::Scalar> {
	type Output = Self;

	#[inline]
	fn add(self, rhs: Self) -> Self::Output {
		let lhs: T::Scalar = self.0;
		let rhs: T::Scalar = rhs.0;

		Self(lhs + rhs)
	}
}

impl<T> Add<&FixedPointValue<T>> for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + Add<T::Scalar, Output = T::Scalar> {
	type Output = Self;

	#[inline]
	fn add(self, rhs: &FixedPointValue<T>) -> Self::Output {
		let lhs: T::Scalar = self.0;
		let rhs: T::Scalar = rhs.0;

		Self(lhs + rhs)
	}
}

impl<T> Sub for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Sub<T::Scalar, Output = T::Scalar> {
	type Output = Self;

	#[inline]
	fn sub(self, rhs: Self) -> Self::Output {
		let lhs: T::Scalar = self.0;
		let rhs: T::Scalar = rhs.0;

		Self(lhs - rhs)
	}
}

impl<T> Sub<&FixedPointValue<T>> for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + Sub<T::Scalar, Output = T::Scalar> {
	type Output = Self;

	#[inline]
	fn sub(self, rhs: &FixedPointValue<T>) -> Self::Output {
		let lhs: T::Scalar = self.0;
		let rhs: T::Scalar = rhs.0;

		Self(lhs - rhs)
	}
}

impl<T> Mul for FixedPointValue<T>
where
    T: Denomination,
    T::Scalar: Eq + ConstZero + NextWidest + Sign + FromPrimitive,
	T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar> {
	type Output = Self;

	#[inline]
	fn mul(self, rhs: Self) -> Self::Output {
		let lhs: T::Scalar = self.0;
		let rhs: T::Scalar = rhs.0;

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		Self::mul(lhs, rhs).map(Self).unwrap()
	}
}

impl<T> Mul<&FixedPointValue<T>> for FixedPointValue<T>
where
    T: Denomination,
    T::Scalar: Copy + Eq + ConstZero + NextWidest + Sign + FromPrimitive,
	T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar> {
	type Output = Self;

	fn mul(self, rhs: &FixedPointValue<T>) -> Self::Output {
		let lhs: T::Scalar = self.0;
		let rhs: T::Scalar = rhs.0;

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		Self::mul(lhs, rhs).map(Self).unwrap()
	}
}

impl<T> Div for FixedPointValue<T>
where
    T: Denomination,
    T::Scalar: Eq + ConstZero + NextWidest + Sign + FromPrimitive,
	T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar> {
	type Output = Self;

	#[inline]
	fn div(self, rhs: Self) -> Self::Output {
		let lhs: T::Scalar = self.0;
		let rhs: T::Scalar = rhs.0;

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		Self::div(lhs, rhs).map(Self).unwrap()
	}
}

impl<T> Div<&FixedPointValue<T>> for FixedPointValue<T>
where
    T: Denomination,
    T::Scalar: Copy + Eq + ConstZero + NextWidest + Sign + FromPrimitive,
	T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar> {
	type Output = Self;

	#[inline]
	fn div(self, rhs: &FixedPointValue<T>) -> Self::Output {
		let lhs: T::Scalar = self.0;
		let rhs: T::Scalar = rhs.0;

		#[allow(clippy::unwrap_used, reason = "unchecked operation")]
		Self::div(lhs, rhs).map(Self).unwrap()
	}
}

impl<T> CheckedAdd for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Add<T::Scalar, Output = T::Scalar> + CheckedAdd + Copy {

	#[inline]
	fn checked_add(&self, v: &Self) -> Option<Self> {
		let lhs: T::Scalar = self.0;
		let rhs: T::Scalar = v.0;

		lhs.checked_add(&rhs).map(Self)
	}
}

impl<T> CheckedSub for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Sub<T::Scalar, Output = T::Scalar> + CheckedSub + Copy {

	#[inline]
	fn checked_sub(&self, v: &Self) -> Option<Self> {
		let lhs: T::Scalar = self.0;
		let rhs: T::Scalar = v.0;

		lhs.checked_sub(&rhs).map(Self)
	}
}

impl<T> CheckedMul for FixedPointValue<T>
where
    T: Denomination,
    T::Scalar: Eq + ConstZero + NextWidest + Sign + FromPrimitive + Copy + CheckedMul,
	T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar> {

	#[inline]
	fn checked_mul(&self, v: &Self) -> Option<Self> {
		let lhs: T::Scalar = self.0;
		let rhs: T::Scalar = v.0;

		Self::mul(lhs, rhs).map(Self).ok()
	}
}

impl<T> CheckedDiv for FixedPointValue<T>
where
    T: Denomination,
    T::Scalar: Eq + ConstZero + NextWidest + Sign + FromPrimitive + Copy + CheckedMul,
	T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
	<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar> {

	#[inline]
	fn checked_div(&self, v: &Self) -> Option<Self> {
		let lhs: T::Scalar = self.0;
		let rhs: T::Scalar = v.0;

		Self::div(lhs, rhs).map(Self).ok()
	}
}

mod borrow {
	use super::*;

	impl<T> Add<FixedPointValue<T>> for &FixedPointValue<T>
	where
		T: Denomination,
		T::Scalar: Copy + Add<T::Scalar, Output = T::Scalar> {
		type Output = FixedPointValue<T>;

		#[inline]
		fn add(self, rhs: FixedPointValue<T>) -> Self::Output {
			let lhs: T::Scalar = self.0;
			let rhs: T::Scalar = rhs.0;

			FixedPointValue(lhs + rhs)
		}
	}

	impl<T> Add<&FixedPointValue<T>> for &FixedPointValue<T>
	where
		T: Denomination,
		T::Scalar: Copy + Add<T::Scalar, Output = T::Scalar> {
		type Output = FixedPointValue<T>;

		#[inline]
		fn add(self, rhs: &FixedPointValue<T>) -> Self::Output {
			let lhs: T::Scalar = self.0;
			let rhs: T::Scalar = rhs.0;

			FixedPointValue(lhs + rhs)
		}
	}

	impl<T> Sub<FixedPointValue<T>> for &FixedPointValue<T>
	where
		T: Denomination,
		T::Scalar: Copy + Sub<T::Scalar, Output = T::Scalar> {
		type Output = FixedPointValue<T>;

		#[inline]
		fn sub(self, rhs: FixedPointValue<T>) -> Self::Output {
			let lhs: T::Scalar = self.0;
			let rhs: T::Scalar = rhs.0;

			FixedPointValue(lhs - rhs)
		}
	}

	impl<T> Sub<&FixedPointValue<T>> for &FixedPointValue<T>
	where
		T: Denomination,
		T::Scalar: Copy + Sub<T::Scalar, Output = T::Scalar> {
		type Output = FixedPointValue<T>;

		#[inline]
		fn sub(self, rhs: &FixedPointValue<T>) -> Self::Output {
			let lhs: T::Scalar = self.0;
			let rhs: T::Scalar = rhs.0;

			FixedPointValue(lhs - rhs)
		}
	}

	impl<T> Mul<FixedPointValue<T>> for &FixedPointValue<T>
	where
	    T: Denomination,
	    T::Scalar: Copy + Eq + ConstZero + NextWidest + Sign + FromPrimitive,
		T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
		<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
		<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar> {
		type Output = FixedPointValue<T>;

		fn mul(self, rhs: FixedPointValue<T>) -> Self::Output {
			let lhs: T::Scalar = self.0;
			let rhs: T::Scalar = rhs.0;

			#[allow(clippy::unwrap_used, reason = "unchecked operation")]
			FixedPointValue(FixedPointValue::<T>::mul(lhs, rhs).unwrap())
		}
	}

	impl<T> Mul<&FixedPointValue<T>> for &FixedPointValue<T>
	where
	    T: Denomination,
	    T::Scalar: Copy + Eq + ConstZero + NextWidest + Sign + FromPrimitive,
		T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
		<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
		<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar> {
		type Output = FixedPointValue<T>;

		fn mul(self, rhs: &FixedPointValue<T>) -> Self::Output {
			let lhs: T::Scalar = self.0;
			let rhs: T::Scalar = rhs.0;

			#[allow(clippy::unwrap_used, reason = "unchecked operation")]
			FixedPointValue(FixedPointValue::<T>::mul(lhs, rhs).unwrap())
		}
	}

	impl<T> Div<FixedPointValue<T>> for &FixedPointValue<T>
	where
	    T: Denomination,
	    T::Scalar: Copy + Eq + ConstZero + NextWidest + Sign + FromPrimitive,
		T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
		<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
		<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar> {
		type Output = FixedPointValue<T>;

		#[inline]
		fn div(self, rhs: FixedPointValue<T>) -> Self::Output {
			let lhs: T::Scalar = self.0;
			let rhs: T::Scalar = rhs.0;

			#[allow(clippy::unwrap_used, reason = "unchecked operation")]
			FixedPointValue(FixedPointValue::<T>::div(lhs, rhs).unwrap())
		}
	}

	impl<T> Div<&FixedPointValue<T>> for &FixedPointValue<T>
	where
	    T: Denomination,
	    T::Scalar: Copy + Eq + ConstZero + NextWidest + Sign + FromPrimitive,
		T::Scalar: TryFrom<<T::Scalar as NextWidest>::Scalar>,
		<T::Scalar as NextWidest>::Scalar: Mul<Output = <T::Scalar as NextWidest>::Scalar>,
		<T::Scalar as NextWidest>::Scalar: Div<Output = <T::Scalar as NextWidest>::Scalar> {
		type Output = FixedPointValue<T>;

		#[inline]
		fn div(self, rhs: &FixedPointValue<T>) -> Self::Output {
			let lhs: T::Scalar = self.0;
			let rhs: T::Scalar = rhs.0;

			#[allow(clippy::unwrap_used, reason = "unchecked operation")]
			FixedPointValue(FixedPointValue::<T>::div(lhs, rhs).unwrap())
		}
	}
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
	use super::*;

	#[rstest::rstest]
	#[case(10_00, 0_50, 5_00)]
	#[case(5_00, 0_25, 1_25)]
	#[case(0_70, 0_10, 0_07)]
	#[case(0_10, 0_30, 0_03)]
	#[case(1_40, 0_10, 0_14)]
	fn mul(#[case] lhs: i64, #[case] rhs: i64, #[case] answer: i64) {
		assert_eq!(FixedPointValue::<With<2, i64>>::mul(lhs, rhs).unwrap(), answer);
	}

	#[rstest::rstest]
	#[case(10_00, 0_50, 20_00)]
	#[case(5_00, 2_00, 2_50)]
	fn div(#[case] lhs: i64, #[case] rhs: i64, #[case] answer: i64) {
		assert_eq!(FixedPointValue::<With<2, i64>>::div(lhs, rhs).unwrap(), answer);
	}

	#[allow(clippy::large_digit_groups)]
	#[rstest::rstest]
	fn div_loop_via_float_conversion() {
		let mut x: I64F9 = I64F9::from_f32(1.0).unwrap();
		let y: I64F9 = I64F9::from_f64(2.0).unwrap();
		let z: I64F9 = I64F9::from_f64(1.0 / 64.0).unwrap();

		x = x / y;

		assert_eq!(x.as_inner(), 0_500000000);

		x = x / y;

		assert_eq!(x.as_inner(), 0_250000000);

		x = x / y;

		assert_eq!(x.as_inner(), 0_125000000);

		x = x / y;

		assert_eq!(x.as_inner(), 0_062500000);

		x = x / y;

		assert_eq!(x.as_inner(), 0_031250000);

		x = x / y;

		assert_eq!(x.as_inner(), 0_015625000);
	}

	#[allow(clippy::large_digit_groups)]
	#[rstest::rstest]
	fn div_loop_via_exact_conversion() {
		let mut x: I64F9 = 1_000000000.as_fixed_point_value();
		let y: I64F9 = 2_000000000.as_fixed_point_value();
		let z: I64F9 = 0_015625000.as_fixed_point_value();

		x = x / y;

		assert_eq!(x.as_inner(), 0_500000000);

		x = x / y;

		assert_eq!(x.as_inner(), 0_250000000);

		x = x / y;

		assert_eq!(x.as_inner(), 0_125000000);

		x = x / y;

		assert_eq!(x.as_inner(), 0_062500000);

		x = x / y;

		assert_eq!(x.as_inner(), 0_031250000);

		x = x / y;

		assert_eq!(x.as_inner(), 0_015625000);
	}
}
