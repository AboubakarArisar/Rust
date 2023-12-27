fn test(mut x:i32) {
    // define a mutable variable
    let mut count = 0;
    // define a while loop
    while x >= 0 { 
       x = x - 3; // decrement the value of x by 3
       count = count + 1;
    }
    println!("{}", count);
}
fn main(){
    print!("Iterations when x = 21 :");
    test(21);
    print!("Iterations when x = 33 :");
    test(33);
}
