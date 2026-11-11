use super::*;

mod arithmetic;
mod conversion;
mod index;
mod index_mut;
mod iter;

pub type Dimension = usize;

pub trait Space<const T: Dimension> {
	type Scalar;
}

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(derive_more::From)]
pub struct Vec<const A: Dimension, B>([B::Scalar; A])
where
	B: Space<A>;

impl<const A: Dimension, B> Vec<A, B>
where
	B: Space<A>,
	B::Scalar: Copy {
	pub fn from_splat(x: B::Scalar) -> Self {
		Self(core::array::from_fn(|_| {
			x
		}))
	}
}

impl<const A: Dimension, B> Default for Vec<A, B>
where
	B: Space<A>,
	B::Scalar: ConstZero {
	fn default() -> Self {
    	Self(core::array::from_fn(|_| {
     		B::Scalar::ZERO
     	}))
	}
}

impl<T> Vec<2, T>
where
	T: Space<2> {
	pub fn new(x: T::Scalar, y: T::Scalar) -> Self {
		Self([x, y])
	}
}

impl<T> Vec<2, T>
where
	T: Space<2> {
	pub fn x(&self) -> &T::Scalar {
		self.0.first().unwrap_or_else(|| unsafe {
			core::hint::unreachable_unchecked()
		})
	}

	pub fn y(&self) -> &T::Scalar {
		self.0.last().unwrap_or_else(|| unsafe {
			core::hint::unreachable_unchecked()
		})
	}
}

impl<T> Vec<3, T>
where
	T: Space<3> {
	pub fn new(
		x: T::Scalar,
		y: T::Scalar,
		z: T::Scalar
	) -> Self {
		Self([x, y, z])
	}
}

impl<T> Vec<3, T>
where
	T: Space<3> {
	pub fn x(&self) -> &T::Scalar {
		self.0.first().unwrap_or_else(|| unsafe {
			core::hint::unreachable_unchecked()
		})
	}

	pub fn y(&self) -> &T::Scalar {
		self.0.get(1).unwrap_or_else(|| unsafe {
			core::hint::unreachable_unchecked()
		})
	}

	pub fn z(&self) -> &T::Scalar {
		self.0.last().unwrap_or_else(|| unsafe {
			core::hint::unreachable_unchecked()
		})
	}
}

pub type Vec2DU8F1 = Vec<2, fpv::U8F1>;
pub type Vec2DU8F2 = Vec<2, fpv::U8F2>;

pub type Vec2DI8F1 = Vec<2, fpv::I8F1>;
pub type Vec2DI8F2 = Vec<2, fpv::I8F2>;

pub type Vec2DU16F1 = Vec<2, fpv::U16F1>;
pub type Vec2DU16F2 = Vec<2, fpv::U16F2>;
pub type Vec2DU16F3 = Vec<2, fpv::U16F3>;
pub type Vec2DU16F4 = Vec<2, fpv::U16F4>;

pub type Vec2DI16F1 = Vec<2, fpv::I16F1>;
pub type Vec2DI16F2 = Vec<2, fpv::I16F2>;
pub type Vec2DI16F3 = Vec<2, fpv::I16F3>;
pub type Vec2DI16F4 = Vec<2, fpv::I16F4>;
