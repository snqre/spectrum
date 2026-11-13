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
//! ### Range
//!
//! The following entries are very educated estimates.
//!
//! > Proper range and documentation will be made available in the future.
//!
//! | Type | Range |
//! | :--- | :---- |
//! | `U8F1`  | `0..=~25.5` |
//! | `U8F2`  | `0..=~2.55` |
//! | `I8F1`  | `~-12.8..=~12.7` |
//! | `I8F2`  | `~-1.28..=~1.27` |
//! | `U16F1` | `0..=~6,553.5` |
//! | `U16F2` | `0..=~655.35` |
//! | `U16F3` | `0..=~65.535` |
//! | `U16F4` | `0..=~6.5535` |
//! | `I16F1` | `~-3,276.8..=~3,276.7` |
//! | `I16F2` | `~-372.68..=~327.67` |
//! | `I16F3` | `~-32.768..=~32.767` |
//! | `I16F4` | `~-3.2768..=~3.2767` |
//! | `U32F1` | `0..=~429,496,729.5` |
//! | `U32F2` | `0..=~42,949,672.95` |
//! | `U32F3` | `0..=~4,294,967.295` |
//! | `U32F4` | `0..=~429,496.7295` |
//! | `U32F5` | `0..=~42,949.67295` |
//! | `U32F6` | `0..=~4,294.967295` |
//! | `U32F7` | `0..=~429.4967295` |
//! | `U32F8` | `0..=~42.94967295` |
//! | `U32F9` | `0..=~4.294967295` |
//! | `I32F1` | `~-214,748,364.8..=~214,748,364.7` |
//! | `I32F2` | `~-21,474,836.48..=~21,474,836.47` |
//! | `I32F3` | `~-2,147,483.648..=~2,147,483.647` |
//! | `I32F4` | `~-214,748.3648..=~214,748.3647` |
//! | `I32F5` | `~-21,474.83648..=~21,474.83647` |
//! | `I32F6` | `~-2,147.483648..=~2,147.483647` |
//! | `I32F7` | `~-214.7483648..=~214.7483647` |
//! | `I32F8` | `~-21.47483648..=~21.47483647` |
//! | `I32F9` | `~-2.147483648..=~2.147483647` |
//! | `U64F1` | `0..=~1,844,674,407,370,955,161.5` |
//! | `U64F2` | `0..=~184,467,440,737,095,516.15` |
//! | `U64F3` | `0..=~18,446,744,073,709,551.615` |
//! | `U64F4` | `0..=~1,844,674,407,370,955.1615` |
//! | `U64F5` | `0..=~184,467,440,737,095.51615` |
//! | `U64F6` | `0..=~18,446,744,073,709.551615` |
//! | `U64F7` | `0..=~1,844,674,407,370.9551615` |
//! | `U64F8` | `0..=~184,467,440,737.09551615` |
//! | `U64F9` | `0..=~18,446,744,073.709551615` |
//! | `U64F10` | `0..=~1,844,674,407.3709551615` |
//! | `U64F11` | `0..=~184,467,440.73709551615` |
//! | `U64F12` | `0..=~18,446,744.073709551615` |
//! | `U64F13` | `0..=~1,844,674.4073709551615` |
//! | `U64F14` | `0..=~184,467,44.073709551615` |
//! | `U64F15` | `0..=~1,844,674.4073709551615` |
//! | `U64F16` | `0..=~184,467.44073709551615` |
//! | `U64F17` | `0..=~18,446.744073709551615` |
//! | `U64F18` | `0..=~1,844.6744073709551615` |
//! | `U64F19` | `0..=~184.46744073709551615` |
//! | `I64F1` | `~-922,337,203,685,477,580.8..=~922,337,203,685,477,580.7`
//! | `I64F2` | `~-92,233,720,368,547,758.08..=~92,233,720,368,547,758.07`
//! | `I64F3` | `~-9,223,372,036,854,775.808..=~9,223,372,036,854,775.807`
//! | `I64F4` | `~-922,337,203,685,477.5808..=~922,337,203,685,477.5807`
//! | `I64F5` | `~-92,233,720,368,547.75808..=~92,233,720,368,547.75807`
//! | `I64F6` | `~-9,223,372,036,854.775808..=~9,223,372,036,854.775807`
//! | `I64F7` | `~-922,337,203,685.4775808..=~922,337,203,685.4775807`
//! | `I64F8` | `~-92,233,720,368.54775808..=~92,233,720,368.54775807`
//! | `I64F9` | `~-9,223,372,036.854775808..=~9,223,372,036.854775807`
//! | `I64F10` | `~-922,337,203.6854775808..=~922,337,203.6854775807`
//! | `I64F11` | `~-92,233,720.36854775808..=~92,233,720.36854775807`
//! | `I64F12` | `~-9,223,372.036854775808..=~9,223,372.036854775807`
//! | `I64F13` | `~-922,337.2036854775808..=~922,337.2036854775807`
//! | `I64F14` | `~-92,233.72036854775808..=~92,233.72036854775807`
//! | `I64F15` | `~-9,223.372036854775808..=~9,223.372036854775807`
//! | `I64F16` | `~-922.3372036854775808..=~922.3372036854775807`
//! | `I64F17` | `~-92.23372036854775808..=~92.23372036854775807`
//! | `I64F18` | `~-9.223372036854775808..=~9.223372036854775807`
//! | `I64F19` | `~-0.9223372036854775808..=~0.9223372036854775807`
//!
//! > `U128` types are currently unstable.
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
