fn main() {
    let mut num:i32 = 3321;
    let sum = calcSum(num);
    println!("Sum of the digits in {} is {}",num , sum);
}

fn calcSum(mut n:i32) -> i32 {
    let mut sum:i32 = 0;
    let mut remainder:i32 = 0;
    while n > 0 {
        remainder = n % 10;
        sum+=remainder;
        n/=10;
    }
    return sum;
}
