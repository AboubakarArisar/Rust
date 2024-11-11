fn factorial(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let n = 5;
    let result = factorial(n);
    println!("The factorial of {} is {}", n, result);
}