use super::*;

impl<const A: Dimension, B> IndexMut<usize> for Vec<A, B>
where
	B: Space<A> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    	&mut self.0[index]
	}
}
