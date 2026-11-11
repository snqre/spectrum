use super::*;

impl<const A: Dimension, B> Add for Vec<A, B>
where
	B: Space<A>,
	B::Scalar: Copy + Add<Output = B::Scalar> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
    	Self(core::array::from_fn(|key| {
     		self.0[key] + rhs.0[key]
     	}))
	}
}

impl<const A: Dimension, B> Sub for Vec<A, B>
where
	B: Space<A>,
	B::Scalar: Copy + Sub<Output = B::Scalar> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
    	Self(core::array::from_fn(|key| {
     		self.0[key] - rhs.0[key]
     	}))
	}
}

impl<const A: Dimension, B> Mul<B::Scalar> for Vec<A, B>
where
	B: Space<A>,
	B::Scalar: Copy + Mul<Output = B::Scalar> {
	type Output = Self;

	fn mul(self, rhs: B::Scalar) -> Self::Output {
    	Self(core::array::from_fn(|key| {
     		self.0[key] * rhs
     	}))
	}
}

impl<const A: Dimension, B> Div<B::Scalar> for Vec<A, B>
where
	B: Space<A>,
	B::Scalar: Copy + Div<Output = B::Scalar> {
	type Output = Self;

	fn div(self, rhs: B::Scalar) -> Self::Output {
    	Self(core::array::from_fn(|key| {
     		self.0[key] / rhs
     	}))
	}
}
