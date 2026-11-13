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

#### Sign

Fixed-point values support sign manipulation directly:

- `to_negative()`: Converts the value to its negative counterpart (returns zero unchanged).
- `to_positive()`: Converts the value to its non-negative/absolute value.


License: MIT OR Apache-2.0
