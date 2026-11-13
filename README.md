

# reliq

## `reliq`

Fixed-point data type collection

### Installation

`Cargo.toml`:

```toml
[dependencies]
reliq = "0.6.0"
```

### Feature

#### Conversion

```rust
use reliq::fpv::I32F9;
use reliq::fpv::conversion::AsFixedPointValue as _:

let value: i32 = 5_000000000;
let value: I32F9 = value.as_fixed_point_value();

let expected: i32 = 5_000000000;

assert_eq!(value.into_inner(), expected);
```

#### `reliq::fpv::abs`

Fixed-point values support sign manipulation directly:

- `FixedPointValue::to_negative()`: Converts the value to its negative counterpart (returns zero unchanged).
- `FixedPointValue::to_positive()`: Converts the value to its non-negative/absolute value.

##### `reliq::fpv::round`

- `FixedPointValue::round_up()`
- `FixedPointValue::round_down()`
- `FixedPointValue::round_towards_zero()`
- `FixedPointValue::round_away_from_zero()`

##### `reliq::fpv::conversion`

```rust
	use reliq::fpv::I64F9;
use reliq::fpv::conversion::AsFixedPointValue as _;

// 5.5
let value: i64 = 5_500000000;
let value: I64F9 = value.as_fixed_point_value();
let value_content: i64 = value.into_inner();

let expected: i64 = 5_500000000;

assert_eq!(value_content, expected);
```

Current version: 0.6.0

Some additional info here

License: MIT OR Apache-2.0
