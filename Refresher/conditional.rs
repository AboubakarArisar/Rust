//example of if else in rust

fn main(){
    let x = 10;
    let condition  = x > 5;

    if condition {
        println!("value of x is greater than 5");
    }
    else{
        println!("value of x is less than 5");
    }



    //if else if else

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    }
    else if number % 3 == 0 {
        println!("number is divisible by 3");
    }
    else if number % 2 == 0 {
        println!("number is divisible by 2");
    }
    else {
        println!("number is not divisible by 4, 3 or 2");
    }
}