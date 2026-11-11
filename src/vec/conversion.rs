use super::*;

impl<T> From<(T::Scalar, T::Scalar)> for Vec<2, T>
where
	T: Space<2> {
	fn from(value: (T::Scalar, T::Scalar)) -> Self {
    	Self::new(value.0, value.1)
	}
}

impl<T> From<(T::Scalar, T::Scalar, T::Scalar)> for Vec<3, T>
where
	T: Space<3> {
	fn from(value: (T::Scalar, T::Scalar, T::Scalar)) -> Self {
		Self::new(value.0, value.1, value.2)
	}
}
