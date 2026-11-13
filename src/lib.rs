//! ## Installation
//!
//! `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! reliq = "0.6.0"
//! ```
//!
//! ## Feature
//!
//! ### Conversion
//!
//! ```rust
//! use reliq::fpv::I32F9;
//! use reliq::fpv::conversion::AsFixedPointValue as _:
//!
//! let value: i32 = 5_000000000;
//! let value: I32F9 = value.as_fixed_point_value();
//!
//! let expected: i32 = 5_000000000;
//!
//! assert_eq!(value.into_inner(), expected);
//! ```
//!
//! ### `reliq::fpv::abs`
//!
//! Fixed-point values support sign manipulation directly:
//!
//! - `FixedPointValue::to_negative()`: Converts the value to its negative counterpart (returns zero unchanged).
//! - `FixedPointValue::to_positive()`: Converts the value to its non-negative/absolute value.
//!
//! #### `reliq::fpv::round`
//!
//! - `FixedPointValue::round_up()`
//! - `FixedPointValue::round_down()`
//! - `FixedPointValue::round_towards_zero()`
//! - `FixedPointValue::round_away_from_zero()`
//!
//! #### `reliq::fpv::conversion`
//!
//! ##### `AsFixedPointValue`
//!
//! Literal conversion from a primitive to a fixed-point value (no scaling applied).
//!
//! ```rust
//! use reliq::fpv::I64F9;
//! use reliq::fpv::conversion::AsFixedPointValue as _;
//!
//! // 5.5
//! let value: i64 = 5_500000000;
//! let value: I64F9 = value.as_fixed_point_value();
//! let value_content: i64 = value.into_inner();
//!
//! let expected: i64 = 5_500000000;
//!
//! assert_eq!(value_content, expected);
//! ```

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
use core::ops::Neg;
use core::ops::Index;
use core::ops::IndexMut;
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

pub mod fpv;
pub mod vec;
