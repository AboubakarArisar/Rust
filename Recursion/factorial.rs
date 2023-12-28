// main function
fn main(){
    // call the function
    let n = 4;
    let fact = factorial(n);
    // print the factorial
    println!("factorial({}): {}", n, fact);
}
// define the factorial function
fn factorial(n: i64) -> i64 {
    if n == 0 { // base case
        1
    }
    else {
        n * factorial(n-1) // recursive case
    }
}
