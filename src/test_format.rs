// This file is intentionally poorly formatted to test rustfmt
fn main() {
    let x = 42;
    if x > 40 {
        println!("big number");
    } else {
        println!("small number");
    }

    let items = vec![1, 2, 3, 4, 5];

    for item in items {
        println!("{}", item);
    }
}
