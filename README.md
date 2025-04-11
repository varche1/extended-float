# Extended Float

A Rust crate providing extended floating-point number implementations with intelligent handling of precision and formatting issues.

## Features

- Wrap native floating-point types with increased precision handling
- Smart string formatting that accounts for floating-point errors
- Complete set of mathematical operators including:
  - Basic arithmetic: +, -, *, /, %
  - Assignment operations: +=, -=, *=, /=, %=
  - Unary operations: -
- Comprehensive equality comparison that handles floating-point peculiarities
- Proper handling of special values (NaN, Infinity)

## Example

```rust
use extended_float::types::ExtendedFloat;

fn main() {
    // Create extended float instances
    let a = ExtendedFloat::from(0.1);
    let b = ExtendedFloat::from(0.2);

    // Calculate the sum
    let sum = a + b;

    // Display with intelligent formatting
    println!("{} + {} = {}", a, b, sum); // Outputs: 0.1 + 0.2 = 0.3

    // Standard floating-point would show: 0.1 + 0.2 = 0.30000000000000004
    println!("{} + {} = {}", 0.1, 0.2, 0.1 + 0.2);

    // Assignment operations
    let mut c = ExtendedFloat::from(5.0);
    c *= ExtendedFloat::from(2.0);
    println!("After multiplication: {}", c); // Outputs: After multiplication: 10
}
```

More examples can be found in the `examples` directory.

# TODO:
* Examples
