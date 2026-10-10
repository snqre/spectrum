#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::nursery)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::correctness)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::missing_errors_doc)]

use core::marker::PhantomData;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Sub;
use core::ops::SubAssign;
use core::ops::Mul;
use core::ops::MulAssign;
use core::ops::Div;
use core::ops::DivAssign;
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

modwire::expose! {
	pub arithmetic
	pub clone
	pub conversion
	pub eq
	pub exponent
	pub fmt
	pub ord
}

pub type Result<T> = core::result::Result<T, Error>;

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
