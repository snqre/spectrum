# Feature
- **Const-Generic Denomination**: Define custom fixed-point scales easily using `With<EXPONENT, Scalar>`.
- **Predefined Aliases**: Hundreds of out-of-the-box type aliases for unsigned (`UxFn`) and signed (`IxFn`) numbers (e.g., `U32F4`, `I64F9`, `U128F38`).

# Installation

`Cargo.toml`:

```toml
[dependencies]
reliq = "0.4.0"
```

# Concept
The core of the library revolves around the `Denomination` trait and the `FixedPointValue<T>` struct.

- `With<const A: Exponent, B>`: Specifies the scale factor (where base is `10^A`) and the underlying integer scalar type (`u8`, `i8`, `u32`, `i64`, `u128`, etc.).
- `FixedPointValue<T>`: Holds the raw scalar while enforcing its fixed-point scale at compile time.

## Example

```rust
use reliq::{I64F2, Result, AsFixedPointValue as _};

fn main() -> Result {
	let x: I64F2 = 10_00.as_fixed_point_value();
	let y: I64F2 = 0_50.as_fixed_point_value();

	let a = x + y; // 10.50
	let b = x - y; //  9.50
	let c = x * y; //  5.00
	let d = x / y; // 20.00
}
```

## Error Handling

- `Error::ArithmeticOverflow`
- `Error::ArithmeticUnderflow`
- `Error::DivisionByZero`

# License

Licensed under the Apache-2.0 license.
