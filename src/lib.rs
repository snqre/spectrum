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
