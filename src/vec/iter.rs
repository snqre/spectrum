use super::*;

impl<const A: Dimension, B> Vec<A, B>
where
	B: Space<A> {
	pub fn iter(&self) -> core::slice::Iter<'_, B::Scalar> {
		self.0.iter()
	}

	pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, B::Scalar> {
		self.0.iter_mut()
	}
}

impl<const A: Dimension, B> IntoIterator for Vec<A, B>
where
	B: Space<A> {
	type Item = B::Scalar;
	type IntoIter = core::array::IntoIter<B::Scalar, A>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl<'a, const A: Dimension, B> IntoIterator for &'a Vec<A, B>
where
	B: Space<A> {
	type Item = &'a B::Scalar;
	type IntoIter = core::slice::Iter<'a, B::Scalar>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a, const A: Dimension, B> IntoIterator for &'a mut Vec<A, B>
where
	B: Space<A> {
	type Item = &'a mut B::Scalar;
	type IntoIter = core::slice::IterMut<'a, B::Scalar>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}
