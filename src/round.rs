use super::*;

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + Eq + PartialOrd,
	T::Scalar: From<u128>,
	T::Scalar: Div<Output = T::Scalar>,
	T::Scalar: Rem<Output = T::Scalar>,
	T::Scalar: ConstOne + ConstZero {
	#[inline]
	#[must_use]
	pub fn round_up(self) -> Self {
		let n: T::Scalar = self.0;
		let base: T::Scalar = T::BASE.into();

		if n % base == T::Scalar::ZERO {
			let n: Self = Self(n);

			return n
		}

		if n >= T::Scalar::ZERO {
			let n: T::Scalar = n / base;
			let n: T::Scalar = n + T::Scalar::ONE;
			let n: T::Scalar = n * base;
			let n: Self = Self(n);

			return n
		}

		let n: T::Scalar = n / base;
		let n: T::Scalar = n * base;
		let n: Self = Self(n);

		n
	}
}

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + Eq + PartialOrd,
	T::Scalar: From<u128>,
	T::Scalar: Sub<Output = T::Scalar>,
	T::Scalar: Div<Output = T::Scalar>,
	T::Scalar: Rem<Output = T::Scalar>,
	T::Scalar: ConstOne + ConstZero {
	#[inline]
	#[must_use]
	pub fn round_down(self) -> Self {
		let n: T::Scalar = self.0;
		let base: T::Scalar = T::BASE.into();

		if n % base == T::Scalar::ZERO {
			let n: Self = Self(n);

			return n
		}

		if n >= T::Scalar::ZERO {
			let n: T::Scalar = n / base;
			let n: T::Scalar = n * base;
			let n: Self = Self(n);

			return n
		}

		let n: T::Scalar = n / base;
		let n: T::Scalar = n - T::Scalar::ONE;
		let n: T::Scalar = n * base;
		let n: Self = Self(n);

		n
	}
}

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + From<u128>,
	T::Scalar: Mul<Output = T::Scalar>,
	T::Scalar: Div<Output = T::Scalar> {
	#[inline]
	#[must_use]
	pub fn round_towards_zero(self) -> Self {
		let base: T::Scalar = T::BASE.into();
		let n: T::Scalar = self.0;
		let n: T::Scalar = n / base;
		let n: T::Scalar = n * base;
		let n: Self = Self(n);

		n
	}
}

impl<T> FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Copy + Eq + PartialOrd,
	T::Scalar: From<u128>,
	T::Scalar: Sub<Output = T::Scalar>,
	T::Scalar: Div<Output = T::Scalar>,
	T::Scalar: Rem<Output = T::Scalar>,
	T::Scalar: ConstOne + ConstZero {
	#[inline]
	#[must_use]
	pub fn round_away_from_zero(self) -> Self {
		let n: T::Scalar = self.0;
		let base: T::Scalar = T::BASE.into();

		if n % base == T::Scalar::ZERO {
			let n: Self = Self(n);

			return n
		}

		if n < T::Scalar::ZERO {
			let n: T::Scalar = n / base;
			let n: T::Scalar = n * base;
			let n: T::Scalar = n - base;
			let n: Self = Self(n);

			return n
		}

		let n: T::Scalar = n / base;
		let n: T::Scalar = n * base;
		let n: T::Scalar = n + base;
		let n: Self = Self(n);

		n
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[rstest::rstest]
	#[case(2_45.as_fixed_point_value())]
	fn round_up(#[case] n: I64F2) {
		todo!()
	}
}
