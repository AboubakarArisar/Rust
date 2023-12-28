fn main() {
    let num = 5;
    let res: i32 = primeChecker(num) as i32;
    if res == 1 {
        println!("{} is prime number" , num);
    } else {
        println!("{}is not prime number",num);
    }
}

fn primeChecker(n: i32) -> bool {
    let mut count: i32 = 0;
    if n == 1 {
        return true;
    } else if n > 1 {
        for i in 1..=n {
            let mut rs: i32 = n % i;
            if rs == 0 {
                count += 1;
            }
        }
        if count == 2 {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}
