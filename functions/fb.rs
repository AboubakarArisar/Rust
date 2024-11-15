fn main(){
    let result = fib(10);
    println!("The result is: {}", result);
}

fn fib(num:i32) -> i32{
    let mut num1 = 0;
    let mut num2 = 1;

    if num == 0{
        return num1;
    }
    else if num == 1{
        return num2;
    }
    else {
       
        for _ in 1..num-2{
            let  num3 = num1 + num2;
            num1 = num2;
            num2 = num3;
        }
        return num2;
    }


}