use super::*;

impl<T> Debug for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: core::fmt::Debug {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("Value")
			.field(&T::EXPONENT)
			.field(&self.0)
			.finish()
	}
}

impl<T> Display for FixedPointValue<T>
where
	T: Denomination,
	T::Scalar: Display {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let s: String = self.0.to_string();
		let exp: usize = T::EXPONENT as usize;

		if exp == 0 {
			return write!(f, "{}", s)
		}

		let (is_negative, digits) = if let Some(s) = s.strip_prefix('-') {
			(true, s)
		} else {
			(false, s.as_str())
		};

		let sign: &str = if is_negative {
			"-"
		} else {
			""
		};

		if digits.len() <= exp {
			let padding: String = "0".repeat(exp - digits.len());

			write!(f, "{}0.{}{}", sign, padding, digits)
		} else {
			let split_position: usize = digits.len() - exp;
			let leading: &str = &digits[..split_position];
			let trailing: &str = &digits[split_position..];

			write!(f, "{}{}.{}", sign, leading, trailing)
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[rstest::rstest]
	#[case(2_00, "2.00")]
	#[case(-30_24, "-30.24")]
	#[case(7_99, "7.99")]
	#[case(0, "0.00")]
	#[case(5, "0.05")]
	#[case(50, "0.50")]
	#[case(-5, "-0.05")]
	#[case(-50, "-0.50")]
	#[case(100, "1.00")]
	#[case(-100, "-1.00")]
	#[case(9999_99, "9999.99")]
	#[case(-9999_99, "-9999.99")]
	fn display(#[case] sample: i64, #[case] answer: &str) {
		assert_eq!(format!("{}", FixedPointValue::<With<2, i64>>(sample)), answer);
	}
}
