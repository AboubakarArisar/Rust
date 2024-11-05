fn main(){
    let n = 10;
    let mut sum = 0;

    for i in 1..=n {
        sum += i;
    }

    println!("Sum of 1 to {} is {}", n, sum);
}