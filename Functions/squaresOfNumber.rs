fn main() {
    println!("Squares of the numbers from 1 to {} is {} ", 5, squares(5));
}

fn squares(n: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..=n {
        sum += i * i;
    }
    return sum;
}
