
# Concept

## `FixedPointValue<T>`



```rust
let x: I64F2 = 10_00.as_scaled();
let y: I64F2 = 0_50.as_scaled();

let sum  = x + y; // 30.50
let diff = x - y; //  9.50
let prod = x * y; //

let x: I64F2 = 10_00.as_scaled();
let y: I64F2 = 0_50.as_scaled();



// directly mul the underlying scalar without scaling
let a: I64F2 = 10_00.as_scaled().raw_mul(00_50.as_scaled());
let b: I64F2 = 10_00.as_scaled().raw_div(00_50.as_scaled());

assert_eq!(a, 50_000.as_scaled())
assert_eq!(b, 20.as_scaled())
```

### Error Handling

- `Error::ArithmeticOverflow`: The operation exceeded the capacity of the underlying scalar type.
-


# License

Apache-2.0
