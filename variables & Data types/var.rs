fn main() {
    let var = 5;
    println!("var: {}", var);

    // In Rust, variables are immutable by default
    let mut a = 5;
    println!("a: {}", a);
    a = 10;
    println!("a: {}", a);

    // Boolean
    let b = true;
    println!("b: {}", b);

    // Character
    let ch = 'c';
    println!("ch: {}", ch);

    // Constant with type f64 for a floating-point number
    const PI: f64 = 3.14;
    println!("PI: {}", PI);
}
