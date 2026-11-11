use super::*;

impl<const A: Dimension, B> Index<usize> for Vec<A, B>
where
	B: Space<A> {
	type Output = B::Scalar;

	fn index(&self, index: usize) -> &Self::Output {
    	&self.0[index]
	}
}
