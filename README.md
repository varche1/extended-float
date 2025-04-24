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

## NaN and Infinity Handling

ExtendedFloat is designed to provide predictable floating-point behavior by rejecting NaN (Not-a-Number) and infinite values. This approach ensures calculations remain well-defined and deterministic, which is critical for financial and trading applications where unexpected floating-point behavior can lead to serious issues.

### Key Principles:

- ExtendedFloat **does not allow NaN or infinite values** at any point
- All API operations will either **reject invalid inputs** or **prevent invalid outputs**
- Operations that would produce NaN or infinity in standard floats will **panic by default**
- **Safe alternatives** are available when panic-free behavior is required

### Creation and Validation

#### Safe Creation Methods:

| Method | Behavior with Invalid Values (NaN/Infinity) | Return Type |
|--------|---------------------------------------------|-------------|
| `new(value)` | **Panics** with descriptive error message | `ExtendedFloat<T>` |
| `try_new(value)` | Returns `None` | `Option<ExtendedFloat<T>>` |
| `try_from_value(value)` | Returns detailed error via `ConversionError` | `Result<ExtendedFloat<T>, ConversionError>` |
| `From<T> for ExtendedFloat<T>` | **Panics** (same as `new()`) | `ExtendedFloat<T>` |

#### Unsafe Creation (Performance-Critical):

```rust
// Caller MUST ensure value is neither NaN nor infinite
unsafe fn new_unchecked(value: T) -> ExtendedFloat<T>
```

### Arithmetic Operations

#### Standard Operators:

All standard arithmetic operators (`+`, `-`, `*`, `/`, `%`, unary `-`) follow this behavior:

- **Inputs:** Operations will panic if either operand contains NaN or infinity
- **Results:** Operations will panic if the result would be NaN or infinity
- **Examples:**
  - Division by zero → **panic** (would produce infinity/NaN)
  - Overflow → **panic** (would produce infinity)
  - Operations on extremely small values → handled safely within floating-point precision

#### Compound Assignment:

Assignment operators (`+=`, `-=`, `*=`, `/=`, `%=`) follow the same validation rules as their non-assignment counterparts.

### Safe Alternatives (Non-Panicking)

For situations where panicking is undesirable, ExtendedFloat implements the `CheckedFloatOps` trait which provides two sets of methods:

#### Option-Returning Methods:

```rust
// Returns None if result would be invalid (NaN/infinity)
fn checked_add(&self, rhs: Self) -> Option<Self>
fn checked_sub(&self, rhs: Self) -> Option<Self>
fn checked_mul(&self, rhs: Self) -> Option<Self>
fn checked_div(&self, rhs: Self) -> Option<Self>
fn checked_rem(&self, rhs: Self) -> Option<Self>
```

#### Result-Returning Methods:

```rust
// Returns specific error information via ConversionError
fn try_add(&self, rhs: Self) -> Result<Self, ConversionError>
fn try_sub(&self, rhs: Self) -> Result<Self, ConversionError>
fn try_mul(&self, rhs: Self) -> Result<Self, ConversionError>
fn try_div(&self, rhs: Self) -> Result<Self, ConversionError>
fn try_rem(&self, rhs: Self) -> Result<Self, ConversionError>
```

These provide error information via the `ConversionError` enum:
- `ConversionError::NaN` - Operation would produce NaN
- `ConversionError::Infinite` - Operation would produce infinity

### Performance Considerations

The default validation on all operations ensures safety but has a performance cost. For performance-critical code paths where validation has already been performed or can be guaranteed through other means:

```rust
// Bypass input validation - UNSAFE!
unsafe fn new_unchecked(value: T) -> Self

// Bypass update validation - UNSAFE!
unsafe fn update_unchecked(&mut self, value: T)
```

**Warning:** Using these unsafe methods creates values that will cause panics in standard operations if they contain NaN or infinity.

### Usage Examples

#### Safe with Panics:

```rust
let a = ExtendedFloat::new(5.0);
let b = ExtendedFloat::new(2.0);
let sum = a + b; // Safe, can't fail

// This would panic:
// let infinity = ExtendedFloat::new(1.0) / ExtendedFloat::new(0.0);
```

#### Safe without Panics:

```rust
use extended_float::traits::CheckedFloatOps;

let a = ExtendedFloat::new(5.0);
let b = ExtendedFloat::new(0.0);

// Using Option
if let Some(result) = a.checked_div(b) {
    println!("Division succeeded: {}", result);
} else {
    println!("Division would produce NaN or infinity");
}

// Using Result with specific error information
match a.try_div(b) {
    Ok(result) => println!("Division succeeded: {}", result),
    Err(ConversionError::NaN) => println!("Division would produce NaN"),
    Err(ConversionError::Infinite) => println!("Division would produce infinity"),
}
```

#### Performance-Critical Code:

```rust
// ONLY when you can guarantee valid values:
let values: Vec<f64> = get_validated_values();
let mut result = ExtendedFloat::new(0.0);

for val in values {
    // Skip validation for performance
    unsafe {
        result += ExtendedFloat::new_unchecked(val);
    }
}
```

# TODO:
* Examples
