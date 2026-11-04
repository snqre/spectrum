#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::nursery)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::correctness)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use core::marker::PhantomData;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Sub;
use core::ops::SubAssign;
use core::ops::Mul;
use core::ops::MulAssign;
use core::ops::Div;
use core::ops::DivAssign;
use core::ops::Rem;
use core::ops::RemAssign;
use core::fmt::Debug;
use core::fmt::Display;
use num_traits::PrimInt;
use num_traits::Float;
use num_traits::ToPrimitive;
use num_traits::AsPrimitive;
use num_traits::FromPrimitive;
use num_traits::CheckedAdd;
use num_traits::CheckedSub;
use num_traits::CheckedMul;
use num_traits::CheckedDiv;
use num_traits::ConstZero;
use num_traits::Zero;
use num_traits::ConstOne;
use num_traits::One;
use num_traits::Bounded;

modwire::expose! {
	pub arithmetic
	pub bound
	pub clone
	pub conversion
	pub eq
	pub exponent
	pub fmt
	pub ord
	pub round
	pub sqrt
	pub zero
}

pub type Result<T = ()> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("arithmetic overflow")]
	ArithmeticOverflow,
	#[error("arithmetic underflow")]
	ArithmeticUnderflow,
	#[error("division by zero")]
	DivisionByZero,
	#[error("unsupported exponent")]
	UnsupportedExponent
}

pub trait Denomination {
	type Scalar;

	const EXPONENT: Exponent;
	const BASE: u128 = 10u128.pow(Self::EXPONENT);

	const F32_RECIPROCAL: f32 = {
		match Self::EXPONENT {
			1 => 1E-1,
			2 => 1E-2,
			3 => 1E-3,
			4 => 1E-4,
			5 => 1E-5,
			6 => 1E-6,
			7 => 1E-7,
			8 => 1E-8,
			9 => 1E-9,
			10 => 1E-10,
			11 => 1E-11,
			12 => 1E-12,
			13 => 1E-13,
			14 => 1E-14,
			15 => 1E-15,
			16 => 1E-16,
			17 => 1E-17,
			18 => 1E-18,
			19 => 1E-19,
			20 => 1E-20,
			21 => 1E-21,
			22 => 1E-22,
			23 => 1E-23,
			24 => 1E-24,
			25 => 1E-25,
			26 => 1E-26,
			27 => 1E-27,
			28 => 1E-28,
			29 => 1E-29,
			30 => 1E-30,
			31 => 1E-31,
			32 => 1E-32,
			33 => 1E-33,
			34 => 1E-34,
			35 => 1E-35,
			36 => 1E-36,
			37 => 1E-37,
			38 => 1E-38,
			_ => panic!()
		}
	};

	const F64_RECIPROCAL: f64 = {
		match Self::EXPONENT {
			1 => 1E-1,
			2 => 1E-2,
			3 => 1E-3,
			4 => 1E-4,
			5 => 1E-5,
			6 => 1E-6,
			7 => 1E-7,
			8 => 1E-8,
			9 => 1E-9,
			10 => 1E-10,
			11 => 1E-11,
			12 => 1E-12,
			13 => 1E-13,
			14 => 1E-14,
			15 => 1E-15,
			16 => 1E-16,
			17 => 1E-17,
			18 => 1E-18,
			19 => 1E-19,
			20 => 1E-20,
			21 => 1E-21,
			22 => 1E-22,
			23 => 1E-23,
			24 => 1E-24,
			25 => 1E-25,
			26 => 1E-26,
			27 => 1E-27,
			28 => 1E-28,
			29 => 1E-29,
			30 => 1E-30,
			31 => 1E-31,
			32 => 1E-32,
			33 => 1E-33,
			34 => 1E-34,
			35 => 1E-35,
			36 => 1E-36,
			37 => 1E-37,
			38 => 1E-38,
			_ => panic!()
		}
	};
}

pub struct With<const A: Exponent, B>(PhantomData<B>);

impl<const A: Exponent, B> Denomination for With<A, B> {
	type Scalar = B;

	const EXPONENT: Exponent = A;
}

pub struct FixedPointValue<T>(T::Scalar)
where
	T: Denomination;

impl<T> FixedPointValue<T>
where
	T: Denomination {
	pub fn into_inner(self) -> T::Scalar {
		self.0
	}
}

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy {
	pub fn as_inner(&self) -> T::Scalar {
		self.0
	}
}

pub type U8F1 = FixedPointValue<With<1, u8>>;
pub type U8F2 = FixedPointValue<With<2, u8>>;

pub type I8F1 = FixedPointValue<With<1, i8>>;
pub type I8F2 = FixedPointValue<With<2, i8>>;

pub type U16F1 = FixedPointValue<With<1, u16>>;
pub type U16F2 = FixedPointValue<With<2, u16>>;
pub type U16F3 = FixedPointValue<With<3, u16>>;
pub type U16F4 = FixedPointValue<With<4, u16>>;

pub type I16F1 = FixedPointValue<With<1, i16>>;
pub type I16F2 = FixedPointValue<With<2, i16>>;
pub type I16F3 = FixedPointValue<With<3, i16>>;
pub type I16F4 = FixedPointValue<With<4, i16>>;

pub type U32F1 = FixedPointValue<With<1, u32>>;
pub type U32F2 = FixedPointValue<With<2, u32>>;
pub type U32F3 = FixedPointValue<With<3, u32>>;
pub type U32F4 = FixedPointValue<With<4, u32>>;
pub type U32F5 = FixedPointValue<With<5, u32>>;
pub type U32F6 = FixedPointValue<With<6, u32>>;
pub type U32F7 = FixedPointValue<With<7, u32>>;
pub type U32F8 = FixedPointValue<With<8, u32>>;
pub type U32F9 = FixedPointValue<With<9, u32>>;

pub type I32F1 = FixedPointValue<With<1, i32>>;
pub type I32F2 = FixedPointValue<With<2, i32>>;
pub type I32F3 = FixedPointValue<With<3, i32>>;
pub type I32F4 = FixedPointValue<With<4, i32>>;
pub type I32F5 = FixedPointValue<With<5, i32>>;
pub type I32F6 = FixedPointValue<With<6, i32>>;
pub type I32F7 = FixedPointValue<With<7, i32>>;
pub type I32F8 = FixedPointValue<With<8, i32>>;
pub type I32F9 = FixedPointValue<With<9, i32>>;

pub type U64F1 = FixedPointValue<With<1, u64>>;
pub type U64F2 = FixedPointValue<With<2, u64>>;
pub type U64F3 = FixedPointValue<With<3, u64>>;
pub type U64F4 = FixedPointValue<With<4, u64>>;
pub type U64F5 = FixedPointValue<With<5, u64>>;
pub type U64F6 = FixedPointValue<With<6, u64>>;
pub type U64F7 = FixedPointValue<With<7, u64>>;
pub type U64F8 = FixedPointValue<With<8, u64>>;
pub type U64F9 = FixedPointValue<With<9, u64>>;
pub type U64F10 = FixedPointValue<With<10, u64>>;
pub type U64F11 = FixedPointValue<With<11, u64>>;
pub type U64F12 = FixedPointValue<With<12, u64>>;
pub type U64F13 = FixedPointValue<With<13, u64>>;
pub type U64F14 = FixedPointValue<With<14, u64>>;
pub type U64F15 = FixedPointValue<With<15, u64>>;
pub type U64F16 = FixedPointValue<With<16, u64>>;
pub type U64F17 = FixedPointValue<With<17, u64>>;
pub type U64F18 = FixedPointValue<With<18, u64>>;
pub type U64F19 = FixedPointValue<With<19, u64>>;

pub type I64F1 = FixedPointValue<With<1, i64>>;
pub type I64F2 = FixedPointValue<With<2, i64>>;
pub type I64F3 = FixedPointValue<With<3, i64>>;
pub type I64F4 = FixedPointValue<With<4, i64>>;
pub type I64F5 = FixedPointValue<With<5, i64>>;
pub type I64F6 = FixedPointValue<With<6, i64>>;
pub type I64F7 = FixedPointValue<With<7, i64>>;
pub type I64F8 = FixedPointValue<With<8, i64>>;
pub type I64F9 = FixedPointValue<With<9, i64>>;
pub type I64F10 = FixedPointValue<With<10, i64>>;
pub type I64F11 = FixedPointValue<With<11, i64>>;
pub type I64F12 = FixedPointValue<With<12, i64>>;
pub type I64F13 = FixedPointValue<With<13, i64>>;
pub type I64F14 = FixedPointValue<With<14, i64>>;
pub type I64F15 = FixedPointValue<With<15, i64>>;
pub type I64F16 = FixedPointValue<With<16, i64>>;
pub type I64F17 = FixedPointValue<With<17, i64>>;
pub type I64F18 = FixedPointValue<With<18, i64>>;
pub type I64F19 = FixedPointValue<With<19, i64>>;

pub type U128F1 = FixedPointValue<With<1, u128>>;
pub type U128F2 = FixedPointValue<With<2, u128>>;
pub type U128F3 = FixedPointValue<With<3, u128>>;
pub type U128F4 = FixedPointValue<With<4, u128>>;
pub type U128F5 = FixedPointValue<With<5, u128>>;
pub type U128F6 = FixedPointValue<With<6, u128>>;
pub type U128F7 = FixedPointValue<With<7, u128>>;
pub type U128F8 = FixedPointValue<With<8, u128>>;
pub type U128F9 = FixedPointValue<With<9, u128>>;
pub type U128F10 = FixedPointValue<With<10, u128>>;
pub type U128F11 = FixedPointValue<With<11, u128>>;
pub type U128F12 = FixedPointValue<With<12, u128>>;
pub type U128F13 = FixedPointValue<With<13, u128>>;
pub type U128F14 = FixedPointValue<With<14, u128>>;
pub type U128F15 = FixedPointValue<With<15, u128>>;
pub type U128F16 = FixedPointValue<With<16, u128>>;
pub type U128F17 = FixedPointValue<With<17, u128>>;
pub type U128F18 = FixedPointValue<With<18, u128>>;
pub type U128F19 = FixedPointValue<With<19, u128>>;
pub type U128F20 = FixedPointValue<With<20, u128>>;
pub type U128F21 = FixedPointValue<With<21, u128>>;
pub type U128F22 = FixedPointValue<With<22, u128>>;
pub type U128F23 = FixedPointValue<With<23, u128>>;
pub type U128F24 = FixedPointValue<With<24, u128>>;
pub type U128F25 = FixedPointValue<With<25, u128>>;
pub type U128F26 = FixedPointValue<With<26, u128>>;
pub type U128F27 = FixedPointValue<With<27, u128>>;
pub type U128F28 = FixedPointValue<With<28, u128>>;
pub type U128F29 = FixedPointValue<With<29, u128>>;
pub type U128F30 = FixedPointValue<With<30, u128>>;
pub type U128F31 = FixedPointValue<With<31, u128>>;
pub type U128F32 = FixedPointValue<With<32, u128>>;
pub type U128F33 = FixedPointValue<With<33, u128>>;
pub type U128F34 = FixedPointValue<With<34, u128>>;
pub type U128F35 = FixedPointValue<With<35, u128>>;
pub type U128F36 = FixedPointValue<With<36, u128>>;
pub type U128F37 = FixedPointValue<With<37, u128>>;
pub type U128F38 = FixedPointValue<With<38, u128>>;

pub type I128F1 = FixedPointValue<With<1, i128>>;
pub type I128F2 = FixedPointValue<With<2, i128>>;
pub type I128F3 = FixedPointValue<With<3, i128>>;
pub type I128F4 = FixedPointValue<With<4, i128>>;
pub type I128F5 = FixedPointValue<With<5, i128>>;
pub type I128F6 = FixedPointValue<With<6, i128>>;
pub type I128F7 = FixedPointValue<With<7, i128>>;
pub type I128F8 = FixedPointValue<With<8, i128>>;
pub type I128F9 = FixedPointValue<With<9, i128>>;
pub type I128F10 = FixedPointValue<With<10, i128>>;
pub type I128F11 = FixedPointValue<With<11, i128>>;
pub type I128F12 = FixedPointValue<With<12, i128>>;
pub type I128F13 = FixedPointValue<With<13, i128>>;
pub type I128F14 = FixedPointValue<With<14, i128>>;
pub type I128F15 = FixedPointValue<With<15, i128>>;
pub type I128F16 = FixedPointValue<With<16, i128>>;
pub type I128F17 = FixedPointValue<With<17, i128>>;
pub type I128F18 = FixedPointValue<With<18, i128>>;
pub type I128F19 = FixedPointValue<With<19, i128>>;
pub type I128F20 = FixedPointValue<With<20, i128>>;
pub type I128F21 = FixedPointValue<With<21, i128>>;
pub type I128F22 = FixedPointValue<With<22, i128>>;
pub type I128F23 = FixedPointValue<With<23, i128>>;
pub type I128F24 = FixedPointValue<With<24, i128>>;
pub type I128F25 = FixedPointValue<With<25, i128>>;
pub type I128F26 = FixedPointValue<With<26, i128>>;
pub type I128F27 = FixedPointValue<With<27, i128>>;
pub type I128F28 = FixedPointValue<With<28, i128>>;
pub type I128F29 = FixedPointValue<With<29, i128>>;
pub type I128F30 = FixedPointValue<With<30, i128>>;
pub type I128F31 = FixedPointValue<With<31, i128>>;
pub type I128F32 = FixedPointValue<With<32, i128>>;
pub type I128F33 = FixedPointValue<With<33, i128>>;
pub type I128F34 = FixedPointValue<With<34, i128>>;
pub type I128F35 = FixedPointValue<With<35, i128>>;
pub type I128F36 = FixedPointValue<With<36, i128>>;
pub type I128F37 = FixedPointValue<With<37, i128>>;
pub type I128F38 = FixedPointValue<With<38, i128>>;




pub type Dimension = usize;

#[derive(Debug)]
pub struct Point<const A: Dimension, B>([B; A]);

impl<T> Point<2, T> {
	pub fn new(x: T, y: T) -> Self {
		Self([x, y])
	}
}

impl<T> Point<2, T> {
	pub fn x(&self) -> &T {
		&self.0.first().expect("at least 2 dimensional")
	}

	pub fn y(&self) -> &T {
		&self.0.last().expect("at least 2 dimensional")
	}
}

pub type Point2DU8F1 = Point<2, U8F1>;
pub type Point2DU8F2 = Point<2, U8F2>;









trait Nm {
	fn from_nm(n: Self) -> Self;
	fn from_um(n: Self) -> Self;
	fn from_mm(n: Self) -> Self;
	fn from_cm(n: Self) -> Self;
	fn from_km(n: Self) -> Self;
	fn from_m(n: Self) -> Self;

	#[must_use]
	fn as_nm(&self) -> Self;

	#[must_use]
	fn as_um(&self) -> Self;

	#[must_use]
	fn as_mm(&self) -> Self;

	#[must_use]
	fn as_cm(&self) -> Self;

	#[must_use]
	fn as_km(&self) -> Self;

	#[must_use]
	fn as_m(&self) -> Self;
}

#[allow(clippy::large_digit_groups)]
#[allow(clippy::unreadable_literal)]
impl Nm for I64F9 {
	fn from_nm(n: Self) -> Self {
		n
	}

	fn from_um(n: Self) -> Self {
		n.as_inner().mul(1000).as_fixed_point_value()
	}

	fn from_mm(n: Self) -> Self {
		n.as_inner().mul(1000000).as_fixed_point_value()
	}

	fn from_cm(n: Self) -> Self {
		n.as_inner().mul(10000000).as_fixed_point_value()
	}

	fn from_km(n: Self) -> Self {
		n.as_inner().mul(1000000000000).as_fixed_point_value()
	}

	fn from_m(n: Self) -> Self {
		n.as_inner().mul(1000000000).as_fixed_point_value()
	}

	fn as_nm(&self) -> Self {
		self.clone()
	}

	fn as_um(&self) -> Self {
		self.as_inner().div_euclid(1000).as_fixed_point_value()
	}

	fn as_mm(&self) -> Self {
		self.as_inner().div_euclid(1000000).as_fixed_point_value()
	}

	fn as_cm(&self) -> Self {
		self.as_inner().div_euclid(10000000).as_fixed_point_value()
	}

	fn as_m(&self) -> Self {
		self.as_inner().div_euclid(1000000000).as_fixed_point_value()
	}

	fn as_km(&self) -> Self {
		self.as_inner().div_euclid(1000000000000).as_fixed_point_value()
	}
}

#[allow(clippy::large_digit_groups)]
#[allow(clippy::unreadable_literal)]
#[cfg(test)]
mod test {
	use super::*;

	#[rstest::rstest]
	#[case(2_540000000, 2540_000000000)]
	fn from_nm(#[case] n: i64, #[case] answer: i64) {
		assert_eq!(I64F9::from_um(n.as_fixed_point_value()), answer.as_fixed_point_value());
	}

	#[test]
	fn as_nm() {
		let n: I64F9 = 20_000000000.as_fixed_point_value();

		assert_eq!(n.as_cm(), 0_000002000.as_fixed_point_value());
	}
}
