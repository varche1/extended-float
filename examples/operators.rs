use extended_float::types::ExtendedFloat;

fn main() {
    // Create some ExtendedFloat values
    let a = ExtendedFloat::from(10.5);
    let b = ExtendedFloat::from(2.5);
    let mut result;

    // Basic operations
    println!("Basic Operations:");
    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {}", a, b, a / b);
    println!("{} % {} = {}", a, b, a % b);
    println!("-{} = {}", a, -a);

    // Assignment operations
    println!("\nAssignment Operations:");

    // Addition assignment
    result = a;
    result += b;
    println!("{} += {} = {}", a, b, result);

    // Subtraction assignment
    result = a;
    result -= b;
    println!("{} -= {} = {}", a, b, result);

    // Multiplication assignment
    result = a;
    result *= b;
    println!("{} *= {} = {}", a, b, result);

    // Division assignment
    result = a;
    result /= b;
    println!("{} /= {} = {}", a, b, result);

    // Remainder assignment
    result = a;
    result %= b;
    println!("{} %= {} = {}", a, b, result);

    // Chained operations
    println!("\nChained Operations:");
    println!("({} + {}) * {} = {}", a, b, b, (a + b) * b);
    println!("{} - {} / {} = {}", a, b, b, a - b / b);

    // Special value handling
    println!("\nSpecial Values:");
    let inf = ExtendedFloat::from(f64::INFINITY);
    let neg_inf = ExtendedFloat::from(f64::NEG_INFINITY);
    let nan = ExtendedFloat::from(f64::NAN);

    println!("{} / 0 = {}", a, a / ExtendedFloat::from(0.0));
    println!("-{} = {}", inf, -inf);
    println!("{} + {} = {}", a, inf, a + inf);
    println!("{} * {} = {}", a, neg_inf, a * neg_inf);
    println!("{} % 0 = {}", a, a % ExtendedFloat::from(0.0));
    println!("NaN: {}", nan);
}
